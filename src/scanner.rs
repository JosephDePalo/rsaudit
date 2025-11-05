use std::fs;
use std::sync::{Arc, Mutex};

use crate::luaregex::LuaRegex;
use mlua::{Function, Lua, Table, prelude::*};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct CheckDefinition {
    id: String,
    name: String,
    description: String,
    severity: String,
    run: Function,
}

type CheckRegistry = Arc<Mutex<Vec<CheckDefinition>>>;

pub type Database = Vec<(String, String, String)>;

pub struct Scanner {
    pub lua: Lua,
    pub registry: CheckRegistry,
}

impl Scanner {
    pub fn new() -> mlua::Result<Self> {
        let lua = Lua::new();

        // Setup Registry and register_check function
        let registry: CheckRegistry = Arc::new(Mutex::new(Vec::new()));
        let registry_clone = Arc::clone(&registry);

        let register_check_fn = lua.create_function_mut(move |_, check_table: Table| {
            let check_def = CheckDefinition {
                id: check_table.get("id")?,
                name: check_table.get("name")?,
                description: check_table.get("description")?,
                severity: check_table.get("severity")?,
                run: check_table.get("run")?,
            };
            let mut registry_guard = registry_clone.lock().unwrap();
            registry_guard.push(check_def);
            Ok(())
        })?;

        // Setup regex passthrough
        let compile_fn = lua
            .create_function(|_, pattern: String| {
                // Compile the regex in Rust
                match Regex::new(&pattern) {
                    // On success, wrap it in our UserData struct and return it
                    Ok(re) => Ok(LuaRegex(re)),
                    // On error, return the error message
                    Err(e) => Err(LuaError::runtime(e.to_string())),
                }
            })
            .unwrap();

        let regex_module = lua.create_table().unwrap();
        regex_module.set("compile", compile_fn).unwrap();

        lua.globals().set("regex", regex_module).unwrap();

        lua.globals().set("register_check", register_check_fn)?;

        Ok(Scanner { lua, registry })
    }

    pub fn load_file(self: &Self, path: &str) -> mlua::Result<()> {
        let lua_code = fs::read_to_string(path)?;
        self.lua.load(&lua_code).exec()?;
        Ok(())
    }

    pub fn list_checks(self: &Self) {
        let registry_guard = self.registry.lock().unwrap();
        for (i, check) in registry_guard.iter().enumerate() {
            println!("{}: {:?}", i, check);
        }
    }

    pub fn run_checks(self: &Self, session: mlua::AnyUserData) -> mlua::Result<Database> {
        let registry_guard = self.registry.lock().unwrap();
        let mut db: Vec<(String, String, String)> = vec![];
        for check in registry_guard.iter() {
            let (status, msg): (String, String) = check.run.call((session.clone(),))?;
            db.push((check.id.clone(), status, msg));
        }
        Ok(db)
    }
}
