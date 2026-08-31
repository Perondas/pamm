use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoUserSettings {
    pub remote: Url,
    #[serde(default)]
    pub local_packs: Vec<String>,
}

impl RepoUserSettings {
    pub fn new(remote: Url) -> Self {
        RepoUserSettings {
            remote,
            local_packs: Vec::default(),
        }
    }

    pub fn get_remote(&self) -> &Url {
        &self.remote
    }
}
