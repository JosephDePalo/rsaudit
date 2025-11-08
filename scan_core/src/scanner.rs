pub mod lua;
pub mod ssh;

use std::sync::Arc;

use anyhow::Result;
use mlua::{Function, Lua, LuaSerdeExt, Value};
use serde::Deserialize;
use tokio::task::JoinHandle;

use crate::{db::Db, scanner::lua::init_lua};
use crate::{
    db::models::{CheckStatus, ScanStatus},
    scanner::ssh::SSHSession,
};

#[derive(Debug, Deserialize)]
struct CheckResult {
    status: CheckStatus,
    details: Option<String>,
}

pub struct Scanner {
    lua: Lua,
    pub db: Db,
}

impl Scanner {
    pub fn new(db: Db) -> Result<Self> {
        let lua = init_lua()?;
        Ok(Self { lua, db })
    }

    pub async fn run(self: &Self) -> Result<()> {
        let rules = self.db.get_all_rules().await?;
        let devices = self.db.get_all_devices().await?;

        let lua = Arc::new(self.lua.clone());
        let db = Arc::new(self.db.clone());
        let rules = Arc::new(rules);

        let mut handles = Vec::new();
        for device in devices {
            let lua = lua.clone();
            let db = db.clone();
            let rules = rules.clone();
            let device = device.clone();

            let handle: JoinHandle<Result<()>> =
                tokio::task::spawn(async move {
                    let session = SSHSession::new(
                        device.address.as_str(),
                        device.username.as_str(),
                        device.password.as_str(),
                    )
                    .await?;

                    lua.globals().set("conn", session)?;

                    let scan =
                        db.add_scan(device.id, ScanStatus::Running).await?;
                    for rule in rules.iter() {
                        let rule_result = async {
                            lua.load(&rule.script_body).exec()?;
                            let func: Function =
                                lua.globals().get("run_check")?;
                            let table: Value = func.call_async(()).await?;
                            let result: CheckResult = lua.from_value(table)?;

                            db.add_scan_result(
                                scan.id,
                                rule.id.clone(),
                                result.status.clone(),
                                result.details.clone(),
                            )
                            .await?;
                            println!("Result: {:?}", &result);
                            anyhow::Ok(())
                        }
                        .await;
                        match rule_result {
                            Ok(_) => (),
                            Err(e) => {
                                db.add_scan_result(
                                    scan.id,
                                    rule.id.clone(),
                                    CheckStatus::Error,
                                    Some(format!(
                                        "Rule execution failed: {}",
                                        e
                                    )),
                                )
                                .await?;
                            }
                        };
                    }
                    db.update_scan_status(scan.id, ScanStatus::Completed)
                        .await?;
                    Ok(())
                });
            handles.push(handle);
        }

        for handle in handles {
            handle.await??;
        }

        Ok(())
    }
}
