use std::collections::HashSet;
use crate::pack::server_info::ServerInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackConfig {
    pub name: String,
    pub description: String,
    pub client_params: Vec<String>,
    pub servers: Vec<ServerInfo>,
    pub parent: Option<String>,
    pub(crate) addons: HashSet<String>,
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
            addons: HashSet::new(),
        }
    }

    pub fn with_addons(mut self, addons: HashSet<String>) -> Self {
        self.addons = addons;
        self
    }
}
