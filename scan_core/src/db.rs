pub mod models;

use crate::db::models::*;

use anyhow::Result;
use sqlx::PgPool;

pub struct Db {
    pub pool: PgPool,
}

impl Db {
    pub async fn new(db_url: &str) -> Result<Self> {
        let pool = PgPool::connect(db_url).await?;
        Ok(Self { pool })
    }

    pub async fn get_all_devices(self: &Self) -> Result<Vec<Device>> {
        let devices = sqlx::query_as!(Device, r"SELECT * FROM devices")
            .fetch_all(&self.pool)
            .await?;
        Ok(devices)
    }

    pub async fn add_device(
        self: &Self,
        address: String,
        username: String,
        password: String,
    ) -> Result<Device> {
        let result = sqlx::query_as!(
            Device,
            r#"
            INSERT INTO devices (address, username, password)
            VALUES ($1, $2, $3)
            RETURNING id, address, username, password
            "#,
            address,
            username,
            password
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result)
    }

    pub async fn get_device(self: &Self, id: i64) -> Result<Option<Device>> {
        let device =
            sqlx::query_as!(Device, r"SELECT * FROM devices WHERE id = $1", id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(device)
    }

    pub async fn remove_device(self: &Self, id: i64) -> Result<u64> {
        let result = sqlx::query!(r"DELETE FROM devices WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}
