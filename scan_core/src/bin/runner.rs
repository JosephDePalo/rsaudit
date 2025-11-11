use anyhow::Result;
use scan_core::{db::Db, scanner::Scanner};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let db_url = env::var("DATABASE_URL")?;
    let db = Db::new(db_url.as_str()).await?;
    let scanner = Scanner::new(db)?;
    scanner.run().await?;

    Ok(())
}
