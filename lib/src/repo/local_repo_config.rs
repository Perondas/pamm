use serde::{Deserialize, Serialize};
use url::Url;

// TODO: rename to something better
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalRepoConfig {
    pub(crate) remote: Url,
    // TODO: Add authentication
}

impl LocalRepoConfig {
    pub fn new(remote: Url) -> Self {
        LocalRepoConfig { remote }
    }

    pub fn get_remote(&self) -> &Url {
        &self.remote
    }
}
