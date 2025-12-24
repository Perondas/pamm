use crate::pack::server_info::ServerInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackConfig {
    pub name: String,
    pub description: String,
    pub client_params: Vec<String>,
    pub servers: Vec<ServerInfo>,
    pub parent: Option<String>,
    pub(crate) addons: HashMap<String, Vec<u8>>,
}

impl PackConfig {
    pub fn new(
        name: String,
        description: String,
        client_params: Vec<String>,
        servers: Vec<ServerInfo>,
        parent: Option<String>,
    ) -> Self {
        Self {
            name,
            description,
            client_params,
            servers,
            parent,
            addons: HashMap::new(),
        }
    }

    pub fn with_addons(mut self, addons: HashMap<String, Vec<u8>>) -> Self {
        self.addons = addons;
        self
    }
}
