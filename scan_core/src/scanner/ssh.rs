use std::sync::Arc;

use async_ssh2_tokio::{AuthMethod, Client};
use mlua::{UserData, UserDataMethods};
use serde::Deserialize;

use anyhow::{Context, Result};

#[derive(Debug, Deserialize)]
pub struct Device {
    pub address: String,
    pub username: String,
    pub password: String,
}

pub struct SSHSession {
    client: Arc<Client>,
}

impl SSHSession {
    pub async fn new(
        addr: &str,
        username: &str,
        password: &str,
    ) -> Result<Self> {
        let authn_method = AuthMethod::with_password(password);
        let client = Client::connect(
            addr,
            username,
            authn_method,
            async_ssh2_tokio::ServerCheckMethod::NoCheck,
        )
        .await
        .context(format!("Failed to establish connection to '{}'", addr))?;

        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub async fn run_cmd(self: &Self, cmd: &str) -> Result<String> {
        Ok(self
            .client
            .execute(cmd)
            .await
            .context(format!("Failed to execute command '{}'", cmd))?
            .stdout)
    }
}

impl UserData for SSHSession {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        // Expose a 'run_command' method to Lua
        methods.add_async_method(
            "run_cmd",
            |_, ssh_session, command: String| async move {
                match ssh_session.run_cmd(command.as_str()).await {
                    Ok(result) => Ok(result),
                    Err(e) => Err(mlua::Error::RuntimeError(format!(
                        "SSH command failed: {}",
                        e
                    ))),
                }
            },
        );
    }
}
