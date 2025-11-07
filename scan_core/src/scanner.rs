pub mod checks;
pub mod lua;
pub mod ssh;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{Context, Result};
use tokio::task::JoinHandle;

use crate::scanner::{
    checks::{CheckResult, CheckRunner},
    ssh::{Device, SSHSession},
};

type ScanResult = HashMap<String, Vec<CheckResult>>;

pub struct Scanner {
    runner: Arc<Mutex<CheckRunner>>,
}

impl Scanner {
    pub fn new() -> Result<Self> {
        let runner = Arc::new(Mutex::new(
            CheckRunner::new().context("Failed to create check runner")?,
        ));

        Ok(Self { runner })
    }

    pub fn load_file(self: &Self, path: &str) -> Result<()> {
        self.runner
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire mutex: {}", e))?
            .load_file(path)?;
        Ok(())
    }

    pub fn exclude_checks(self: &Self, ids: &Vec<String>) -> Result<()> {
        self.runner
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire mutex: {}", e))?
            .exclude_checks(ids)?;
        Ok(())
    }

    pub fn clear_runner(self: &mut Self) -> Result<()> {
        self.runner
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire mutex: {}", e))?
            .clear()?;
        Ok(())
    }

    pub async fn scan_devices(
        self: &mut Self,
        devices: Vec<Device>,
    ) -> Result<ScanResult> {
        let mut handles = Vec::new();
        let runner = Arc::new(
            self.runner
                .lock()
                .map_err(|e| anyhow::anyhow!("Failed to acquire mutex: {}", e))?
                .clone(),
        );

        for device in devices {
            let runner = runner.clone();
            let handle: JoinHandle<Result<(String, Vec<CheckResult>)>> =
                tokio::task::spawn(async move {
                    let session = SSHSession::new(
                        device.address.as_str(),
                        device.username.as_str(),
                        device.password.as_str(),
                    )
                    .await?;
                    let session_userdata =
                        runner.lua.create_userdata(session)?;
                    Ok((
                        device.address,
                        runner.run_checks(session_userdata).await?,
                    ))
                });
            handles.push(handle);
        }

        let mut db: ScanResult = HashMap::new();
        for handle in handles {
            if let Ok(handle) = handle.await {
                match handle {
                    Ok((address, result)) => {
                        db.insert(address, result);
                    }
                    Err(e) => eprintln!("An error occurred: {}", e),
                }
            }
        }

        Ok(db)
    }
}
