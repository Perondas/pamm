use crate::handle::repo_handle::RepoHandle;
use crate::models::pack::pack_config::PackConfig;
use anyhow::ensure;

pub trait UpdatePack {
    fn update_pack(&self, pack_config: &PackConfig) -> anyhow::Result<()>;
}

impl UpdatePack for RepoHandle {
    fn update_pack(&self, pack_config: &PackConfig) -> anyhow::Result<()> {
        ensure!(
            self.repo_config.packs.contains(&pack_config.name),
            "Pack '{}' not found in repo",
            pack_config.name
        );

        self.write_named(pack_config, &pack_config.name)
    }
}
