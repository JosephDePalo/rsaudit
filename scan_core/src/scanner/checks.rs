use std::fs;
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use mlua::{Function, Lua};
use serde::{Deserialize, Serialize};

use crate::scanner::lua::init_lua;

#[derive(Debug, Clone)]
pub struct CheckDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: String,
    pub run: Function,
}

pub type CheckRegistry = Arc<Mutex<Vec<CheckDefinition>>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct CheckResult {
    id: String,
    name: String,
    description: String,
    severity: String,
    compliant: bool,
    message: String,
}

impl CheckResult {
    pub fn from_check(
        check: &CheckDefinition,
        compliance: bool,
        msg: String,
    ) -> Self {
        Self {
            id: check.id.clone(),
            name: check.name.clone(),
            description: check.description.clone(),
            severity: check.severity.clone(),
            compliant: compliance,
            message: if msg != "" { msg } else { String::from("") },
        }
    }
}

#[derive(Clone)]
pub struct CheckRunner {
    pub lua: Lua,
    pub registry: CheckRegistry,
}

impl CheckRunner {
    pub fn new() -> Result<Self> {
        let registry: CheckRegistry = Arc::new(Mutex::new(Vec::new()));

        let lua = init_lua(&registry)
            .context("Could not initialize Lua interpreter")?;

        Ok(CheckRunner { lua, registry })
    }

    pub fn clear(self: &mut Self) -> Result<()> {
        self.registry
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire mutex: {}", e))?
            .clear();
        self.lua = init_lua(&self.registry)
            .context("Could not reinitialize Lua interpreter")?;

        Ok(())
    }

    pub fn load_file(self: &Self, path: &str) -> mlua::Result<()> {
        let lua_code = fs::read_to_string(path)?;
        self.lua.load(&lua_code).exec()?;
        Ok(())
    }

    pub async fn run_checks(
        self: &Self,
        session: mlua::AnyUserData,
    ) -> Result<Vec<CheckResult>> {
        let checks = {
            self.registry
                .lock()
                .map_err(|e| anyhow::anyhow!("Failed to acquire mutex: {}", e))?
                .clone()
        };
        let mut db: Vec<CheckResult> = vec![];
        for check in checks.iter() {
            let (status, msg): (bool, String) =
                check.run.call_async((session.clone(),)).await?;
            db.push(CheckResult::from_check(check, status, msg));
        }
        Ok(db)
    }

    pub fn exclude_checks(self: &Self, ids: &Vec<String>) -> Result<()> {
        let mut registry_guard = self
            .registry
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire mutex: {}", e))?;
        registry_guard.retain(|c| !ids.contains(&c.id));
        Ok(())
    }
}
