use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteConfig {
    pub(crate) remote: Url,
    // TODO: Add authentication
}

impl RemoteConfig {
    pub fn new(remote: Url) -> Self {
        RemoteConfig { remote }
    }
    
    pub fn get_remote(&self) -> &Url {
        &self.remote
    }
}