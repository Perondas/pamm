use crate::application::repo::handle::ConcreteRepoHandle;
use crate::application::repo::repo_base::RepoBase;
use crate::domain::repo::repo_config::RepoConfig;
use crate::domain::repo::server_config::ServerConfig;
use crate::infrastructure::fs::files::fs_readable::KnownFSReadable;
use crate::infrastructure::fs::files::name_consts::WWW_DIR_NAME;
use anyhow::Context;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

#[cfg(feature = "deploy")]
pub mod missions;

#[derive(Debug)]
pub struct ServerRepoHandle {
    base: RepoBase,
    #[allow(dead_code)]
    pub(crate) server_config: ServerConfig,
}

impl ServerRepoHandle {
    pub async fn open(repo_path: &Path) -> anyhow::Result<Self> {
        let base = RepoBase::load(repo_path).await?;
        let server_config = ServerConfig::read_from_known(repo_path)
            .await
            .context("Failed to read server config. Is this a server repository?")?;
        Ok(Self {
            base,
            server_config,
        })
    }

    pub async fn create(parent_path: &Path, repo_config: RepoConfig) -> anyhow::Result<Self> {
        let repo_path = repo_config
            .init_server_repo_blank_on_fs(parent_path)
            .await?;

        Self::open(&repo_path)
            .await
            .context("Failed to open ServerRepoHandle after creating repo")
    }

    pub(crate) fn get_www_path(&self) -> PathBuf {
        self.base.repo_path.join(WWW_DIR_NAME)
    }
}

impl Deref for ServerRepoHandle {
    type Target = RepoBase;
    fn deref(&self) -> &RepoBase {
        &self.base
    }
}

impl DerefMut for ServerRepoHandle {
    fn deref_mut(&mut self) -> &mut RepoBase {
        &mut self.base
    }
}

impl ConcreteRepoHandle for ServerRepoHandle {}
