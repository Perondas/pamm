use crate::handle::repo_handle::RepoHandle;
use crate::io::files::file_paths::rel_path::RelPath;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_user_settings::PackUserSettings;
use anyhow::{anyhow, ensure, Context};

#[cfg_attr(test, mockall::automock)]
pub trait GetPack {
    fn get_pack(&self, pack_name: &str) -> anyhow::Result<PackConfig>;
    fn get_pack_with_settings(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<(PackConfig, PackUserSettings)>;
}

impl GetPack for RepoHandle {
    fn get_pack(&self, pack_name: &str) -> anyhow::Result<PackConfig> {
        ensure!(
            self.repo_config.packs.contains(pack_name),
            "Pack '{}' not found in repo",
            pack_name
        );

        self.read_known::<PackConfig>(&RelPath::from_name(pack_name))
            .context(anyhow!(
                "Failed to read pack config for {} in {:#?}",
                pack_name,
                self.repo_path
            ))
    }
    fn get_pack_with_settings(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<(PackConfig, PackUserSettings)> {
        let pack_config = self.get_pack(pack_name)?;

        let pack_user_settings =
            self.read_known(&RelPath::from_name(pack_name))
                .context(anyhow!(
                    "Failed to read settings for {} in {:#?}",
                    pack_name,
                    self.repo_path
                ))?;

        Ok((pack_config, pack_user_settings))
    }
}
