use crate::pack::config::server_info::ServerInfo;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackConfig {
    pub(crate) name: String,
    pub(crate) description: String,
    pub client_params: Vec<String>,
    pub(crate) servers: Vec<ServerInfo>,
    pub(crate) remote: Option<Url>,
}

impl PackConfig {
    pub fn with_remote(self, remote: Url) -> Self {
        PackConfig {
            remote: Some(remote),
            ..self
        }
    }

    pub fn get_remote(&self) -> Option<&Url> {
        self.remote.as_ref()
    }

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
            remote: None,
        }
    }
}
