use crate::handle::repo_handle::RepoHandle;
use crate::io::files::file_paths::rel_path::RelPath;
use crate::models::repo::repo_config::RepoConfig;

pub trait UpdateRepoConfig {
    fn update_repo_config(&mut self, repo_config: RepoConfig) -> anyhow::Result<()>;
}

impl UpdateRepoConfig for RepoHandle {
    fn update_repo_config(&mut self, repo_config: RepoConfig) -> anyhow::Result<()> {
        self.write(&RelPath::new(), &repo_config)?;
        self.repo_config = repo_config;
        Ok(())
    }
}
