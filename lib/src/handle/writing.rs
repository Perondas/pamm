use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::fs_writable::{KnownFSWritable, NamedFSWritable};
use crate::models::pack::pack_config::PackConfig;
use anyhow::ensure;

impl RepoHandle {
    pub fn add_pack(&mut self, pack_config: &PackConfig) -> anyhow::Result<()> {
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

    fn write_named<T: NamedFSWritable>(&self, value: &T, identifier: &str) -> anyhow::Result<()> {
        value.write_to_named(&self.repo_path, identifier)
    }

    fn write<T: KnownFSWritable>(&self, value: &T) -> anyhow::Result<()> {
        value.write_to(&self.repo_path)
    }
}
