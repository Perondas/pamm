use crate::application::repo::handle::{ConcreteRepoHandle, RepoBase};
use crate::domain::repo::repo_config::RepoConfig;
use crate::domain::repo::server_config::ServerConfig;
use crate::infrastructure::filesystem::files::fs_readable::KnownFSReadable;
use crate::infrastructure::filesystem::files::name_consts::WWW_DIR_NAME;
use anyhow::Context;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

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

    pub fn get_www_path(&self) -> PathBuf {
        self.base.repo_path.join(WWW_DIR_NAME)
    }

    /// Returns a list of all packs in the repository.
    pub fn get_all_packs(&self) -> Vec<&str> {
        self.repo_config.packs.iter().map(|s| s.as_str()).collect()
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
