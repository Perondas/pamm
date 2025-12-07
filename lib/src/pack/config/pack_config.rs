use crate::pack::config::server_info::ServerInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackConfig {
    pub name: String,
    pub description: String,
    pub client_params: Vec<String>,
    pub servers: Vec<ServerInfo>,
}

impl PackConfig {
    pub fn new(
        name: String,
        description: String,
        client_params: Vec<String>,
        servers: Vec<ServerInfo>,
    ) -> Self {
        PackConfig {
            name,
            description,
            client_params,
            servers,
        }
    }
}
