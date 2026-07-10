pub mod add_pack;
pub mod delete_pack;
pub mod save_pack_settings;

pub mod update_pack;
pub mod update_repo_config;

use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::fs_writable::{KnownFSWritable, NamedFSWritable};
use crate::models::identifiable::Identifiable;
use anyhow::{Context, anyhow};

impl RepoHandle {
    pub(in crate::handle) fn write_named<T: NamedFSWritable>(
        &self,
        value: &T,
        identifier: &str,
    ) -> anyhow::Result<()> {
        let path = self.repo_path.join(T::get_rel_path(identifier));
        // Per-pack files nest in a pack folder that may not exist yet.
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating directory {:?}", parent))?;
        }
        value
            .write_to_path(path)
            .context(anyhow!("writing {:?}", identifier))
    }

    pub(in crate::handle) fn write_identifiable<T: NamedFSWritable + Identifiable>(
        &self,
        value: &T,
    ) -> anyhow::Result<()> {
        self.write_named(value, value.get_identifier())
    }

    pub(in crate::handle) fn write<T: KnownFSWritable>(&self, value: &T) -> anyhow::Result<()> {
        value.write_to(&self.repo_path)
    }
}
