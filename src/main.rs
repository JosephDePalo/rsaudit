use std::io::Read;
use std::sync::Arc;
use std::{fs, io};

use anyhow::Result;
use clap::Parser;
use rsaudit::checks::{CheckResult, CheckRunner};
use rsaudit::config::{Args, Config};
use rsaudit::sshsession::SSHSession;
use std::collections::HashMap;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let contents = if let Some(path) = args.config {
        fs::read_to_string(path)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    let config: Config = toml::from_str(&contents)?;
    let scanner = Arc::new(CheckRunner::new()?);
    for file_path in &config.settings.check_files {
        scanner.load_file(file_path)?;
    }
    scanner.exclude_checks(&config.settings.exclusion_ids)?;
    let mut handles = Vec::new();
    for device in config.devices {
        let scanner = scanner.clone();
        let handle: JoinHandle<Result<(String, Vec<CheckResult>)>> =
            tokio::task::spawn(async move {
                let session = SSHSession::new(
                    device.address.as_str(),
                    device.username.as_str(),
                    device.password.as_str(),
                )
                .await?;
                let session_userdata = scanner.lua.create_userdata(session)?;
                Ok((
                    device.address,
                    scanner.run_checks(session_userdata).await?,
                ))
            });
        handles.push(handle);
    }
    let mut db: HashMap<String, Vec<CheckResult>> = HashMap::new();
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
    let json = serde_json::to_string_pretty(&db)?;
    println!("{}", json);
    Ok(())
}
