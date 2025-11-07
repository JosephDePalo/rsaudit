use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Device {
    pub id: i32,
    pub address: String,
    pub username: String,
    pub password: String,
}
