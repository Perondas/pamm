use crate::application::repo::handle::{ConcreteRepoHandle, RepoHandle};
use crate::application::repo::repo_base::RepoBase;
use crate::domain::repo::repo_config::RepoConfig;

pub trait UpdateRepoConfig {
    fn update_repo_config(
        &mut self,
        repo_config: RepoConfig,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

impl UpdateRepoConfig for RepoBase {
    async fn update_repo_config(&mut self, repo_config: RepoConfig) -> anyhow::Result<()> {
        self.write(&repo_config).await?;
        self.repo_config = repo_config;
        Ok(())
    }
}

impl<T: ConcreteRepoHandle> UpdateRepoConfig for T {
    async fn update_repo_config(&mut self, repo_config: RepoConfig) -> anyhow::Result<()> {
        self.deref_mut().update_repo_config(repo_config).await
    }
}

impl UpdateRepoConfig for RepoHandle {
    async fn update_repo_config(&mut self, repo_config: RepoConfig) -> anyhow::Result<()> {
        match self {
            #[cfg(feature = "client")]
            Self::Client(handle) => handle.update_repo_config(repo_config).await,
            #[cfg(feature = "server")]
            Self::Server(handle) => handle.update_repo_config(repo_config).await,
        }
    }
}
