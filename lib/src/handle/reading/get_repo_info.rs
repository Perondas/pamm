use crate::handle::repo_handle::RepoHandle;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;
use anyhow::anyhow;
use std::path::Path;

pub trait GetRepoInfo {
    fn get_repo_path(&self) -> &Path;
    fn get_config(&self) -> &RepoConfig;
    fn get_repo_user_settings(&self) -> anyhow::Result<&RepoUserSettings>;
}

impl GetRepoInfo for RepoHandle {
    fn get_repo_path(&self) -> &Path {
        &self.repo_path
    }

    fn get_config(&self) -> &RepoConfig {
        &self.repo_config
    }
    fn get_repo_user_settings(&self) -> anyhow::Result<&RepoUserSettings> {
        self.repo_user_settings
            .as_ref()
            .ok_or(anyhow!("Repo user settings not found"))
    }
}
