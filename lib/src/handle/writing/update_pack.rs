use crate::handle::repo_handle::{ConcreteRepoHandle, RepoBase, RepoHandle};
use crate::models::pack::pack_config::PackConfig;
use anyhow::ensure;

pub trait UpdatePack {
    fn update_pack(
        &self,
        pack_config: &PackConfig,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

impl UpdatePack for RepoBase {
    async fn update_pack(&self, pack_config: &PackConfig) -> anyhow::Result<()> {
        ensure!(
            self.repo_config.packs.contains(&pack_config.name),
            "Pack '{}' not found in repo",
            pack_config.name
        );

        self.write(pack_config).await
    }
}

impl<T: ConcreteRepoHandle> UpdatePack for T {
    async fn update_pack(&self, pack_config: &PackConfig) -> anyhow::Result<()> {
        self.deref().update_pack(pack_config).await
    }
}

impl UpdatePack for RepoHandle {
    async fn update_pack(&self, pack_config: &PackConfig) -> anyhow::Result<()> {
        match self {
            #[cfg(feature = "client")]
            Self::Client(handle) => handle.update_pack(pack_config).await,
            #[cfg(feature = "server")]
            Self::Server(handle) => handle.update_pack(pack_config).await,
        }
    }
}
