use crate::pack::server_info::ServerInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PackManifest {
    pub addons: Vec<String>,
    pub pack_checksum: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackConfig {
    pub name: String,
    pub description: String,
    pub client_params: String,
    pub servers: Vec<ServerInfo>,
    pub remote: Option<String>,
}
