use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub password: String,
    pub uses_battle_eye: bool,
}