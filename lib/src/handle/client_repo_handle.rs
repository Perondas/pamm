use crate::handle::reading::get_pack::GetPack;
use crate::handle::repo_handle::RepoHandle;
use crate::handle::writing::save_pack_settings::SavePackSettings;
use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::fs::migrations::run_migrations;
use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::files::file_paths::rel_path::RelPath;
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
        run_migrations(repo_path, &base.repo_config)?;
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

impl SavePackSettings for ClientRepoHandle {
    fn save_pack_settings(
        &self,
        pack_name: &str,
        settings: &PackUserSettings,
    ) -> anyhow::Result<()> {
        self.write(&RelPath::from_name(pack_name), settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::repo::repo_version::RepoVersion;
    use crate::util::test_utils::TestTempDir;
    use std::collections::HashSet;
    use std::fs;

    // Opening a legacy flat (v1) client repo migrates it to the per-pack
    // layout, including hoisting indexes/ out of the addon dir.
    #[test]
    fn open_migrates_v1_client_repo() {
        let tmp = TestTempDir::new("pamm_client_open_migrates_v1");
        let repo_path = tmp.path().join("repo");
        fs::create_dir_all(&repo_path).unwrap();

        let mut packs = HashSet::new();
        packs.insert("core".to_string());
        let repo_config = RepoConfig::new("repo".to_string(), "desc".to_string(), packs);
        crate::io::fs::fs_writable::KnownFSWritable::write_to(&repo_config, &repo_path).unwrap();
        let settings = RepoUserSettings::new(Url::parse("http://localhost/").unwrap());
        crate::io::fs::fs_writable::KnownFSWritable::write_to(&settings, &repo_path).unwrap();

        // Flat v1 client layout.
        fs::write(repo_path.join("core.pack.config.json"), b"{}").unwrap();
        fs::write(repo_path.join("core.pack.settings.json"), b"{}").unwrap();
        fs::create_dir_all(repo_path.join("core_pack_addons/@addon1")).unwrap();
        fs::create_dir_all(repo_path.join("core_pack_addons/indexes")).unwrap();
        fs::write(
            repo_path.join("core_pack_addons/indexes/checksum_index.pamm"),
            b"idx",
        )
        .unwrap();

        ClientRepoHandle::open(&repo_path).unwrap();

        assert!(repo_path.join("core/pack.config.json").is_file());
        assert!(repo_path.join("core/pack.settings.json").is_file());
        assert!(repo_path.join("core/addons/@addon1").is_dir());
        assert!(repo_path.join("core/indexes/checksum_index.pamm").is_file());
        assert!(!repo_path.join("core_pack_addons").exists());
        assert_eq!(
            RepoVersion::read_or_v1(&repo_path).unwrap(),
            RepoVersion::current()
        );
    }
}
