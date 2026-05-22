use crate::handle::repo_handle::RepoHandle;
use crate::models::repo::repo_config::RepoConfig;
use std::path::Path;

pub trait GetRepoInfo {
    fn get_repo_path(&self) -> &Path;
    fn get_config(&self) -> &RepoConfig;
}

impl GetRepoInfo for RepoHandle {
    fn get_repo_path(&self) -> &Path {
        &self.repo_path
    }

    fn get_config(&self) -> &RepoConfig {
        &self.repo_config
    }
}
