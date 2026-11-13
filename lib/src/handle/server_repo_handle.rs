use crate::handle::repo_handle::RepoHandle;
use crate::io::files::name_consts::WWW_DIR_NAME;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::server_config::ServerConfig;
use anyhow::Context;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ServerRepoHandle {
    base: RepoHandle,
    #[allow(dead_code)]
    pub(crate) server_config: ServerConfig,
}

impl ServerRepoHandle {
    pub async fn create(parent_path: &Path, repo_config: RepoConfig) -> anyhow::Result<Self> {
        let repo_path = repo_config
            .init_server_repo_blank_on_fs(parent_path)
            .await?;

        let base = RepoHandle::open(&repo_path).await?;

        base.try_into()
            .context("Failed to convert RepoHandle to ServerRepoHandle after creating repo")
    }

    pub fn get_www_path(&self) -> PathBuf {
        self.base.repo_path.join(WWW_DIR_NAME)
    }

    /// Returns a list of all packs in the repository.
    pub fn get_all_packs(&self) -> Vec<&str> {
        self.repo_config.packs.iter().map(|s| s.as_str()).collect()
    }
}

impl Deref for ServerRepoHandle {
    type Target = RepoHandle;
    fn deref(&self) -> &RepoHandle {
        &self.base
    }
}

impl DerefMut for ServerRepoHandle {
    fn deref_mut(&mut self) -> &mut RepoHandle {
        &mut self.base
    }
}

impl TryFrom<RepoHandle> for ServerRepoHandle {
    type Error = anyhow::Error;

    fn try_from(base: RepoHandle) -> Result<Self, Self::Error> {
        match &base.repo_type {
            crate::handle::repo_handle::RepoType::Server { server_config } => Ok(Self {
                server_config: server_config.clone(),
                base,
            }),
            _ => Err(anyhow::anyhow!("RepoHandle is not a server repo")),
        }
    }
}
