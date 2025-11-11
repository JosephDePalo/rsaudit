pub mod crypto;
pub mod models;

use crate::db::models::*;

use anyhow::Result;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct Db {
    pub pool: PgPool,
}

impl Db {
    pub async fn new(db_url: &str) -> Result<Self> {
        let pool = PgPool::connect(db_url).await?;
        Ok(Self { pool })
    }

    // --- Device CRUD ---

    pub async fn get_all_devices(self: &Self) -> Result<Vec<Device>> {
        let devices = sqlx::query_as!(
            Device,
            r"SELECT id, address, username, password FROM devices"
        )
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
            sqlx::query_as!(Device, r"SELECT id, address, username, password FROM devices WHERE id = $1", id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(device)
    }

    pub async fn update_device(
        self: &Self,
        id: i64,
        address: String,
        username: String,
        password: String,
    ) -> Result<Device> {
        let device = sqlx::query_as!(
            Device,
            r#"
            UPDATE devices
            SET address = $2, username = $3, password = $4
            WHERE id = $1
            RETURNING id, address, username, password
            "#,
            id,
            address,
            username,
            password
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(device)
    }

    pub async fn remove_device(self: &Self, id: i64) -> Result<u64> {
        let result = sqlx::query!(r"DELETE FROM devices WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    // --- Rule CRUD ---

    pub async fn get_all_rules(self: &Self) -> Result<Vec<Rule>> {
        let rules = sqlx::query_as!(
            Rule,
            r#"
            SELECT 
                id,
                name,
                description,
                severity as "severity: SeverityLevel",
                check_type as "check_type: CheckType",
                script_body
            FROM rules
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rules)
    }

    pub async fn get_rule(self: &Self, id: String) -> Result<Rule> {
        let result = sqlx::query_as!(
            Rule,
            r#"
            SELECT
                id,
                name,
                description,
                severity as "severity: SeverityLevel",
                check_type as "check_type: CheckType",
                script_body
            FROM rules
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result)
    }

    pub async fn add_rule(
        self: &Self,
        id: String,
        name: String,
        description: Option<String>,
        severity: SeverityLevel,
        check_type: CheckType,
        script_body: String,
    ) -> Result<Rule> {
        let result = sqlx::query_as!(
            Rule,
            r#"
            INSERT INTO rules
                (id, name, description, severity, check_type, script_body)
            VALUES
                ($1, $2, $3, $4::severity_level, $5::check_type, $6)
            RETURNING
                id, 
                name, 
                description, 
                severity as "severity: SeverityLevel", 
                check_type as "check_type: CheckType",
                script_body
            "#,
            id,
            name,
            description,
            severity as _,
            check_type as _,
            script_body
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result)
    }

    pub async fn update_rule(
        self: &Self,
        id: String,
        name: String,
        description: Option<String>,
        severity: SeverityLevel,
        check_type: CheckType,
        script_body: String,
    ) -> Result<Rule> {
        let result = sqlx::query_as!(
            Rule,
            r#"
            UPDATE rules
            SET
                name = $2,
                description = $3,
                severity = $4::severity_level,
                check_type = $5::check_type,
                script_body = $6
            WHERE id = $1
            RETURNING
                id, 
                name, 
                description, 
                severity as "severity: SeverityLevel", 
                check_type as "check_type: CheckType",
                script_body
            "#,
            id,
            name,
            description,
            severity as _,
            check_type as _,
            script_body
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result)
    }

    pub async fn remove_rule(self: &Self, id: String) -> Result<u64> {
        let result = sqlx::query!(r"DELETE FROM rules WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    // --- Scan CRUD ---

    pub async fn add_scan(
        self: &Self,
        device_id: i64,
        status: ScanStatus,
    ) -> Result<Scan> {
        let scan = sqlx::query_as!(
            Scan,
            r#"
            INSERT INTO scans (device_id, status)
            VALUES ($1, $2::scan_status)
            RETURNING id, device_id, status as "status: ScanStatus"
            "#,
            device_id,
            status as _
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(scan)
    }

    pub async fn get_scan(self: &Self, id: i64) -> Result<Option<Scan>> {
        let scan = sqlx::query_as!(
            Scan,
            r#"
            SELECT id, device_id, status as "status: ScanStatus"
            FROM scans WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(scan)
    }

    pub async fn update_scan_status(
        self: &Self,
        id: i64,
        status: ScanStatus,
    ) -> Result<Scan> {
        let scan = sqlx::query_as!(
            Scan,
            r#"
            UPDATE scans SET status = $2::scan_status
            WHERE id = $1
            RETURNING id, device_id, status as "status: ScanStatus"
            "#,
            id,
            status as _
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(scan)
    }

    pub async fn get_scans_for_device(
        self: &Self,
        device_id: i64,
    ) -> Result<Vec<Scan>> {
        let scans = sqlx::query_as!(
            Scan,
            r#"
            SELECT id, device_id, status as "status: ScanStatus"
            FROM scans WHERE device_id = $1
            "#,
            device_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(scans)
    }

    // --- ScanResult CRUD ---

    pub async fn add_scan_result(
        self: &Self,
        scan_id: i64,
        rule_id: String,
        status: CheckStatus,
        details: Option<String>,
    ) -> Result<ScanResult> {
        let result = sqlx::query_as!(
            ScanResult,
            r#"
            INSERT INTO scan_results (scan_id, rule_id, status, details)
            VALUES ($1, $2, $3::check_status, $4)
            RETURNING id, scan_id, rule_id, status as "status: CheckStatus", details
            "#,
            scan_id,
            rule_id,
            status as _,
            details
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result)
    }

    pub async fn get_scan_results_for_scan(
        self: &Self,
        scan_id: i64,
    ) -> Result<Vec<ScanResult>> {
        let results = sqlx::query_as!(
            ScanResult,
            r#"
            SELECT id, scan_id, rule_id, status as "status: CheckStatus", details
            FROM scan_results WHERE scan_id = $1
            "#,
            scan_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(results)
    }
}
