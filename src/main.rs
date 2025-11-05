use std::fs;

use rsaudit::config::Config;
use rsaudit::scanner::{Database, Scanner};
use rsaudit::sshsession::SSHSession;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let toml_str = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&toml_str).unwrap();
    let scanner = Scanner::new().unwrap();
    scanner.load_file("scripts/my_checks.lua").unwrap();
    for device in config.devices {
        let session = SSHSession::new(
            device.address.as_str(),
            device.username.as_str(),
            device.password.as_str(),
        );
        let session_userdata = scanner.lua.create_userdata(session.clone())?;
        let db: Database = scanner.run_checks(session_userdata)?;
        println!("DB: {:?}", db);
    }
    Ok(())
}
