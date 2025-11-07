use serde::Deserialize;
use sqlx::{FromRow, Type};

#[derive(Type, Debug)]
#[sqlx(type_name = "severity_level", rename_all = "lowercase")]
pub enum SeverityLevel {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Type, Debug)]
#[sqlx(type_name = "check_type", rename_all = "lowercase")]
pub enum CheckType {
    Lua,
}

#[derive(Type, Debug)]
#[sqlx(type_name = "check_status", rename_all = "lowercase")]
pub enum CheckStatus {
    Pass,
    Fail,
    Error,
}

#[derive(Type, Debug)]
#[sqlx(type_name = "scan_status", rename_all = "lowercase")]
pub enum ScanStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct Device {
    pub id: i64,
    pub address: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, FromRow)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: SeverityLevel,
    pub check_type: CheckType,
    pub script_body: String,
}

#[derive(Debug, FromRow)]
pub struct Scan {
    pub id: i64,
    pub device_id: i64,
    pub status: ScanStatus,
}

#[derive(Debug, FromRow)]
pub struct ScanResult {
    pub id: i64,
    pub scan_id: i64,
    pub rule_id: String,
    pub status: CheckStatus,
    pub details: String,
}
