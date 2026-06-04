use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::handle::repo_handle::RepoHandle;
use crate::io::name_consts::WWW_DIR_NAME;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_user_settings::PackUserSettings;
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

impl GetPack for ServerRepoHandle {
    fn get_pack(&self, pack_name: &str) -> anyhow::Result<PackConfig> {
        self.base.get_pack(pack_name)
    }

    fn get_pack_with_settings(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<(PackConfig, PackUserSettings)> {
        self.base.get_pack_with_settings(pack_name)
    }
}

impl GetRepoInfo for ServerRepoHandle {
    fn get_repo_path(&self) -> &Path {
        self.base.get_repo_path()
    }

    fn get_config(&self) -> &RepoConfig {
        self.base.get_config()
    }
}
