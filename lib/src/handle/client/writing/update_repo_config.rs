use crate::handle::client::client_repo_handle::ClientRepoHandle;
use crate::models::repo::repo_config::RepoConfig;

pub trait UpdateRepoConfig {
    fn update_repo_config(&mut self, repo_config: RepoConfig) -> anyhow::Result<()>;
}

impl UpdateRepoConfig for ClientRepoHandle {
    fn update_repo_config(&mut self, repo_config: RepoConfig) -> anyhow::Result<()> {
        self.write(&repo_config)?;
        self.repo_config = repo_config;
        Ok(())
    }
}
