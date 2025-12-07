use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoConfig {
    pub(crate) name: String,
    pub(crate) description: String,
    /// Store the names of packs available in this repo
    pub(crate) packs: Vec<String>,
}