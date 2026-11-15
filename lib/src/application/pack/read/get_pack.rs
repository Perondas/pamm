use crate::application::repo::handle::{RepoBase, RepoHandle};
#[cfg(feature = "server")]
use crate::application::repo::server::ServerRepoHandle;
use crate::domain::pack::pack_config::PackConfig;
use crate::domain::pack::pack_user_settings::PackUserSettings;
use anyhow::{Context, anyhow, ensure};

pub trait GetPack {
    fn get_pack(&self, pack_name: &str) -> impl Future<Output = anyhow::Result<PackConfig>> + Send;
    fn get_pack_with_settings(
        &self,
        pack_name: &str,
    ) -> impl Future<Output = anyhow::Result<(PackConfig, PackUserSettings)>> + Send;

    fn get_all_pack_names(&self) -> impl Future<Output = anyhow::Result<Vec<&str>>> + Send;
}

impl RepoBase {
    /// Read a pack's config off disk without checking that the pack is registered.
    pub(in crate::application) async fn read_pack_config(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<PackConfig> {
        self.read_keyed::<PackConfig>(pack_name)
            .await
            .context(anyhow!(
                "Failed to read pack config for {} in {:#?}",
                pack_name,
                self.repo_path
            ))
    }

    /// Read pack user settings without checking the pack registry.
    pub(in crate::application) async fn read_pack_settings(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<PackUserSettings> {
        self.read_keyed(pack_name).await.context(anyhow!(
            "Failed to read settings for {} in {:#?}",
            pack_name,
            self.repo_path
        ))
    }

    pub(in crate::application) async fn get_registered_pack(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<PackConfig> {
        ensure!(
            self.repo_config.packs.contains(pack_name),
            "Pack '{}' not found in repo",
            pack_name
        );

        self.read_pack_config(pack_name).await
    }

    pub(in crate::application) async fn get_registered_pack_with_settings(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<(PackConfig, PackUserSettings)> {
        let pack_config = self.get_registered_pack(pack_name).await?;
        let pack_user_settings = self.read_pack_settings(pack_name).await?;
        Ok((pack_config, pack_user_settings))
    }

    pub(in crate::application) fn get_registered_pack_names(&self) -> Vec<&str> {
        self.repo_config.packs.iter().map(|s| s.as_str()).collect()
    }
}

#[cfg(feature = "server")]
impl GetPack for ServerRepoHandle {
    async fn get_pack(&self, pack_name: &str) -> anyhow::Result<PackConfig> {
        self.get_registered_pack(pack_name).await
    }

    async fn get_pack_with_settings(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<(PackConfig, PackUserSettings)> {
        self.get_registered_pack_with_settings(pack_name).await
    }

    async fn get_all_pack_names(&self) -> anyhow::Result<Vec<&str>> {
        Ok(self.get_registered_pack_names())
    }
}

impl GetPack for RepoHandle {
    async fn get_pack(&self, pack_name: &str) -> anyhow::Result<PackConfig> {
        match self {
            #[cfg(feature = "client")]
            Self::Client(handle) => handle.get_pack(pack_name).await,
            #[cfg(feature = "server")]
            Self::Server(handle) => handle.get_pack(pack_name).await,
        }
    }

    async fn get_pack_with_settings(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<(PackConfig, PackUserSettings)> {
        match self {
            #[cfg(feature = "client")]
            Self::Client(handle) => handle.get_pack_with_settings(pack_name).await,
            #[cfg(feature = "server")]
            Self::Server(handle) => handle.get_pack_with_settings(pack_name).await,
        }
    }

    async fn get_all_pack_names(&self) -> anyhow::Result<Vec<&str>> {
        match self {
            #[cfg(feature = "client")]
            Self::Client(handle) => handle.get_all_pack_names().await,
            #[cfg(feature = "server")]
            Self::Server(handle) => handle.get_all_pack_names().await,
        }
    }
}
