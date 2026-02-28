use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::fs_readable::NamedFSReadable;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_user_settings::PackUserSettings;
use crate::models::repo::repo_config::RepoConfig;
use anyhow::{Context, anyhow, ensure};

impl RepoHandle {
    pub fn get_config(&self) -> &RepoConfig {
        &self.repo_config
    }

    pub fn get_pack(&self, pack_name: &str) -> anyhow::Result<PackConfig> {
        ensure!(
            self.repo_config.packs.contains(pack_name),
            "Pack '{}' not found in repo",
            pack_name
        );

        self.read_named::<PackConfig>(pack_name).context(anyhow!(
            "Failed to read pack config for {} in {:#?}",
            pack_name,
            self.repo_path
        ))
    }

    pub fn get_pack_with_settings(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<(PackConfig, PackUserSettings)> {
        let pack_config = self.get_pack(pack_name)?;

        let pack_user_settings = self.read_named(pack_name).context(anyhow!(
            "Failed to read settings for {} in {:#?}",
            pack_name,
            self.repo_path
        ))?;

        Ok((pack_config, pack_user_settings))
    }

    fn read_named<T: NamedFSReadable>(&self, identifier: &str) -> anyhow::Result<T> {
        T::read_from_named(&self.repo_path, identifier)
    }
}
