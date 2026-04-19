pub mod add_pack;
pub mod apply_diff;
pub mod delete_pack;
pub mod update_pack;
pub mod update_repo_config;

use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::fs_writable::{KnownFSWritable, NamedFSWritable};

impl RepoHandle {
    pub(in crate::handle) fn write_named<T: NamedFSWritable>(
        &self,
        value: &T,
        identifier: &str,
    ) -> anyhow::Result<()> {
        value.write_to_named(&self.repo_path, identifier)
    }

    pub(in crate::handle) fn write<T: KnownFSWritable>(&self, value: &T) -> anyhow::Result<()> {
        value.write_to(&self.repo_path)
    }
}
