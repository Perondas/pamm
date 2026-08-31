use crate::handle::repo_handle::RepoHandle;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_user_settings::PackUserSettings;
use anyhow::{Context, anyhow, ensure};

#[cfg_attr(test, mockall::automock)]
pub trait GetPack {
    fn get_pack(&self, pack_name: &str) -> anyhow::Result<PackConfig>;
    fn get_pack_with_settings(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<(PackConfig, PackUserSettings)>;
}

impl RepoHandle {
    /// Read a pack's config off disk **without** checking that the pack is
    /// registered in `repo.config.json`. Only for callers that have already
    /// established the pack exists by other means — a client's local packs are
    /// deliberately absent from the repo config, which mirrors the remote.
    pub(in crate::handle) fn read_pack_config(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<PackConfig> {
        self.read_keyed::<PackConfig>(pack_name).context(anyhow!(
            "Failed to read pack config for {} in {:#?}",
            pack_name,
            self.repo_path
        ))
    }

    /// Read a pack's user settings off disk, without the registry check. See
    /// [`RepoHandle::read_pack_config`].
    pub(in crate::handle) fn read_pack_settings(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<PackUserSettings> {
        self.read_keyed(pack_name).context(anyhow!(
            "Failed to read settings for {} in {:#?}",
            pack_name,
            self.repo_path
        ))
    }
}

impl GetPack for RepoHandle {
    fn get_pack(&self, pack_name: &str) -> anyhow::Result<PackConfig> {
        ensure!(
            self.repo_config.packs.contains(pack_name),
            "Pack '{}' not found in repo",
            pack_name
        );

        self.read_pack_config(pack_name)
    }
    fn get_pack_with_settings(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<(PackConfig, PackUserSettings)> {
        let pack_config = self.get_pack(pack_name)?;

        let pack_user_settings = self.read_pack_settings(pack_name)?;

        Ok((pack_config, pack_user_settings))
    }
}
