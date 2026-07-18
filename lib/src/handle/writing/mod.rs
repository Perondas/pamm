pub mod add_pack;
pub mod delete_pack;
pub mod save_pack_settings;

pub mod update_pack;
pub mod update_repo_config;

use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::fs_writable::{KnownFSWritable, NamedFSWritable};
use crate::io::files::file_paths::rel_path::RelPath;
use crate::models::keyed::Keyed;
use anyhow::{anyhow, Context};

impl RepoHandle {
    pub(in crate::handle) fn write_named<T: NamedFSWritable>(
        &self,
        value: &T,
        rel_path: &RelPath,
        identifier: &str,
    ) -> anyhow::Result<()> {
        let path = rel_path
            .with_base_path(&self.repo_path)
            .join(T::file_name(identifier));
        // Per-pack files nest in a pack folder that may not exist yet.
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating directory {:?}", parent))?;
        }
        value
            .write_to_path(path)
            .context(anyhow!("writing {:?}", identifier))
    }

    pub(in crate::handle) fn write_identifiable<T: NamedFSWritable + Keyed>(
        &self,
        rel_path: &RelPath,
        value: &T,
    ) -> anyhow::Result<()> {
        self.write_named(value, rel_path, value.get_key())
    }

    pub(in crate::handle) fn write<T: KnownFSWritable>(
        &self,
        rel_path: &RelPath,
        value: &T,
    ) -> anyhow::Result<()> {
        let path = rel_path
            .with_base_path(&self.repo_path)
            .join(T::file_name());
        value.write_to_path(&path)
    }
}
