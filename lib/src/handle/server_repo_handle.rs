use crate::handle::repo_handle::RepoHandle;
use crate::io::name_consts::WWW_DIR_NAME;
use crate::models::repo::repo_config::RepoConfig;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ServerRepoHandle {
    base: RepoHandle,
}

impl ServerRepoHandle {
    pub fn open(repo_path: &Path) -> anyhow::Result<Self> {
        let base = RepoHandle::open(repo_path)?;
        Ok(Self { base })
    }

    pub fn create(parent_path: &Path, repo_config: RepoConfig) -> anyhow::Result<Self> {
        let base = RepoHandle::create_repo_folder(parent_path, repo_config)?;
        Ok(Self { base })
    }

    pub fn get_www_path(&self) -> PathBuf {
        self.base.repo_path.join(WWW_DIR_NAME)
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
