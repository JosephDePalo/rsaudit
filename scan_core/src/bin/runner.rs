use anyhow::Result;
use scan_core::{db::Db, scanner::Scanner};

#[tokio::main]
async fn main() -> Result<()> {
    let db_url = "postgresql://joe@localhost/complier";
    let db = Db::new(db_url).await?;
    let scanner = Scanner::new(db)?;
    scanner.run().await?;

    Ok(())
}
