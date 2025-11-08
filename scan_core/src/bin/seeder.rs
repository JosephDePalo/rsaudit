use std::{env, fs};

use anyhow::Result;
use dotenvy::dotenv;
use mlua::{Lua, LuaSerdeExt, Table};

use scan_core::db::{
    Db,
    models::{CheckType, SeverityLevel},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RuleMetadata {
    id: String,
    name: String,
    description: Option<String>,
    severity: SeverityLevel,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("Environment variable 'DATABASE_URL' not set");
    let db = Db::new(db_url.as_str()).await?;

    let lua = Lua::new();

    for path in args {
        let code = fs::read_to_string(path)?;
        lua.load(&code).exec()?;
        let meta_table: Table = lua.globals().get("METADATA")?;
        let meta: RuleMetadata =
            lua.from_value(mlua::Value::Table(meta_table))?;
        db.add_rule(
            meta.id.clone(),
            meta.name,
            meta.description,
            meta.severity,
            CheckType::Lua,
            code,
        )
        .await?;
        println!("Added '{}'", meta.id);
    }

    Ok(())
}
