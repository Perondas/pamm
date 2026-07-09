use crate::handle::repo_handle::RepoHandle;
use crate::models::repo::repo_config::RepoConfig;
use std::path::Path;

pub trait GetRepoInfo {
    /// Returns the repo root path exactly as it was supplied when the handle was
    /// opened/created (not canonicalized): absolute if the caller passed an
    /// absolute path, otherwise relative to the process working directory.
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
