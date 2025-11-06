use std::sync::Arc;

use async_ssh2_tokio::{AuthMethod, Client};
use mlua::{UserData, UserDataMethods};

pub struct SSHSession {
    client: Arc<Client>,
}

impl SSHSession {
    pub async fn new(addr: &str, username: &str, password: &str) -> Self {
        let authn_method = AuthMethod::with_password(password);
        let client = Client::connect(
            addr,
            username,
            authn_method,
            async_ssh2_tokio::ServerCheckMethod::NoCheck,
        )
        .await
        .unwrap();

        Self {
            client: Arc::new(client),
        }
    }

    pub async fn run_cmd(self: &Self, cmd: &str) -> String {
        self.client.execute(cmd).await.unwrap().stdout
    }
}

impl UserData for SSHSession {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        // Expose a 'run_command' method to Lua
        methods.add_async_method(
            "run_cmd",
            |_, ssh_session, command: String| async move {
                Ok(ssh_session.run_cmd(command.as_str()).await)
            },
        );
    }
}
