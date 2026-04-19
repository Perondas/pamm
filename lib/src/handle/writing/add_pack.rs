use crate::handle::repo_handle::RepoHandle;
use crate::models::pack::pack_config::PackConfig;
use anyhow::ensure;

pub trait AddPack {
    fn add_pack(&mut self, pack_config: &PackConfig) -> anyhow::Result<()>;
}

impl AddPack for RepoHandle {
    fn add_pack(&mut self, pack_config: &PackConfig) -> anyhow::Result<()> {
        ensure!(
            !self.repo_config.packs.contains(&pack_config.name),
            "Pack '{}' already exists in repo",
            pack_config.name
        );

        self.repo_config.packs.insert(pack_config.name.clone());
        self.write(&self.repo_config)?;

        self.write_named(pack_config, &pack_config.name)?;

        pack_config.init_blank_on_fs(&self.repo_path)?;

        Ok(())
    }
}
