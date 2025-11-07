use clap::Parser;
use serde::Deserialize;

use scan_core::scanner::ssh::Device;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub devices: Vec<Device>,
    pub settings: Settings,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub exclusion_ids: Vec<String>,
    pub check_files: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(name = "scan_core", version, about = "CLI compliance scanning tool")]
pub struct Args {
    #[arg(short, long)]
    pub config: Option<String>,
}
