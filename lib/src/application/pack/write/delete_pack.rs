use crate::application::repo::handle::{ConcreteRepoHandle, RepoBase, RepoHandle};
use crate::infrastructure::filesystem::files::file_paths::rel_path::RelPath;
use crate::infrastructure::filesystem::files::util::path_checks::is_dir;
use anyhow::{Context, anyhow, ensure};

pub trait DeletePack {
    fn delete_pack(
        &mut self,
        pack_name: &str,
        delete_on_fs: bool,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

impl DeletePack for RepoBase {
    async fn delete_pack(&mut self, pack_name: &str, _delete_on_fs: bool) -> anyhow::Result<()> {
        ensure!(
            self.repo_config.packs.contains(pack_name),
            "Pack '{}' not found in repo",
            pack_name
        );

        let _pack_path = RelPath::from_name(pack_name);
        self.repo_config.packs.remove(pack_name);
        self.write(&self.repo_config).await?;

        let pack_dir = self.repo_path.join(pack_name);
        if is_dir(&pack_dir).await {
            tokio::fs::remove_dir_all(&pack_dir)
                .await
                .context(anyhow!("Failed to remove empty pack folder"))?;
        }

        Ok(())
    }
}

impl<T: ConcreteRepoHandle> DeletePack for T {
    async fn delete_pack(&mut self, pack_name: &str, delete_on_fs: bool) -> anyhow::Result<()> {
        self.deref_mut().delete_pack(pack_name, delete_on_fs).await
    }
}

impl DeletePack for RepoHandle {
    async fn delete_pack(&mut self, pack_name: &str, delete_on_fs: bool) -> anyhow::Result<()> {
        match self {
            #[cfg(feature = "client")]
            Self::Client(handle) => handle.delete_pack(pack_name, delete_on_fs).await,
            #[cfg(feature = "server")]
            Self::Server(handle) => handle.delete_pack(pack_name, delete_on_fs).await,
        }
    }
}
