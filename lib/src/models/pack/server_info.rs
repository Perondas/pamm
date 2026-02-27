use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub password: String,
    pub uses_battle_eye: bool,
}

impl Default for ServerInfo {
    fn default() -> Self {
        ServerInfo {
            name: "My Arma 3 Server".to_string(),
            address: "bestserver.ever".to_string(),
            port: 2302,
            password: "".to_string(),
            uses_battle_eye: true,
        }
    }
}
