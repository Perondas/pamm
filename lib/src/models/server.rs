use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ServerConfig {
    pub server_dir: Option<PathBuf>,
}

impl ServerConfig {
    pub(crate) fn new(server_dir: Option<PathBuf>) -> Self {
        Self { server_dir }
    }
}
