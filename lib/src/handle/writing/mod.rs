pub mod add_pack;
pub mod delete_pack;
pub mod save_pack_settings;

pub mod update_pack;
pub mod update_repo_config;

use crate::handle::repo_handle::RepoHandle;
use crate::io::files::file_paths::keyed_path::KeyedFilePath;
use crate::io::files::file_paths::self_identified_path::SelfIdentifiedFilePath;
use crate::io::fs::fs_writable::FixedFsWritable;

impl RepoHandle {
    pub(in crate::handle) fn write<T: FixedFsWritable + SelfIdentifiedFilePath>(
        &self,
        value: &T,
    ) -> anyhow::Result<()> {
        let path = value.file_path().with_base_path(&self.repo_path);
        value.write_fixed(&path)
    }

    pub(in crate::handle) fn write_keyed<T: FixedFsWritable + KeyedFilePath>(
        &self,
        value: &T,
        key: &str,
    ) -> anyhow::Result<()> {
        let path = T::file_path(key).with_base_path(&self.repo_path);
        value.write_fixed(&path)
    }
}
