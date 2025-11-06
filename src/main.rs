use std::io::Read;
use std::sync::Arc;
use std::{fs, io};

use clap::Parser;
use rsaudit::config::{Args, Config};
use rsaudit::scanner::{Database, Scanner};
use rsaudit::sshsession::SSHSession;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let contents = if let Some(path) = args.config {
        fs::read_to_string(path)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    let config: Config = toml::from_str(&contents).unwrap();
    let scanner = Arc::new(Scanner::new().unwrap());
    for file_path in &config.settings.check_files {
        scanner.load_file(file_path).unwrap();
    }
    scanner.exclude_checks(&config.settings.exclusion_ids);
    let mut handles = Vec::new();
    for device in config.devices {
        let scanner = scanner.clone();
        let handle = tokio::task::spawn(async move {
            let session = SSHSession::new(
                device.address.as_str(),
                device.username.as_str(),
                device.password.as_str(),
            )
            .await;
            let session_userdata =
                scanner.lua.create_userdata(session).unwrap();
            (
                device.address,
                scanner.async_run_checks(session_userdata).await.unwrap(),
            )
        });
        handles.push(handle);
    }
    let mut db: HashMap<String, Database> = HashMap::new();
    for handle in handles {
        if let Ok((address, result)) = handle.await {
            db.insert(address, result);
        }
    }
    let json = serde_json::to_string_pretty(&db)?;
    println!("{}", json);
    Ok(())
}
