use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::handle::repo_handle::RepoHandle;
use crate::handle::writing::save_pack_settings::SavePackSettings;
use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::known_file::KnownFile;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_user_settings::PackUserSettings;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;
use anyhow::ensure;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use url::Url;

#[derive(Debug)]
pub struct ClientRepoHandle {
    base: RepoHandle,
    user_settings: RepoUserSettings,
}

impl ClientRepoHandle {
    pub fn open(repo_path: &Path) -> anyhow::Result<Self> {
        let base = RepoHandle::open(repo_path)?;
        ensure!(
            repo_path.join(RepoUserSettings::file_name()).exists(),
            "Client repo at {:?} is missing user.repo.settings.json. \
             Did you mean to initialize from a remote via `init-remote`?",
            repo_path
        );
        let user_settings = RepoUserSettings::read_from_known(repo_path)?;
        Ok(Self {
            base,
            user_settings,
        })
    }

    pub fn init_from_remote(dest_path: &Path, remote: &Url) -> anyhow::Result<Self> {
        ensure!(dest_path.is_dir(), "Destination path is not a folder");

        let (repo_config, user_settings) = RepoConfig::init_from_remote(dest_path, remote)?;

        let repo_path = dest_path.join(&repo_config.name);
        let base = RepoHandle::from_parts(repo_path, repo_config);

        Ok(Self {
            base,
            user_settings,
        })
    }

    pub fn user_settings(&self) -> &RepoUserSettings {
        &self.user_settings
    }

    pub fn remote(&self) -> &Url {
        self.user_settings.get_remote()
    }
}

impl Deref for ClientRepoHandle {
    type Target = RepoHandle;
    fn deref(&self) -> &RepoHandle {
        &self.base
    }
}

impl DerefMut for ClientRepoHandle {
    fn deref_mut(&mut self) -> &mut RepoHandle {
        &mut self.base
    }
}

impl GetPack for ClientRepoHandle {
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

impl GetRepoInfo for ClientRepoHandle {
    fn get_repo_path(&self) -> &Path {
        self.base.get_repo_path()
    }

    fn get_config(&self) -> &RepoConfig {
        self.base.get_config()
    }
}

impl SavePackSettings for ClientRepoHandle {
    fn save_pack_settings(
        &self,
        pack_name: &str,
        settings: &PackUserSettings,
    ) -> anyhow::Result<()> {
        self.write_named(settings, pack_name)
    }
}
