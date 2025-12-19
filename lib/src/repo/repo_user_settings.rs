use crate::{hr_serializable, known_name};
use serde::{Deserialize, Serialize};
use url::Url;

hr_serializable!(RepoUserSettings);
known_name!(RepoUserSettings, "user.repo.settings.json");
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoUserSettings {
    pub(crate) remote: Url,
    // TODO: Add authentication
}

impl RepoUserSettings {
    pub fn new(remote: Url) -> Self {
        RepoUserSettings { remote }
    }

    pub fn get_remote(&self) -> &Url {
        &self.remote
    }
}
