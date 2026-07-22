pub mod get_canonical_addon_paths;
#[cfg(target_os = "linux")]
pub mod get_linux_addon_paths;
pub mod get_pack;
pub mod get_repo_info;

use crate::handle::repo_handle::RepoHandle;
use crate::io::files::file_paths::keyed_path::KeyedFilePath;
use crate::io::fs::fs_readable::KnownFSReadable;
use anyhow::{anyhow, Context};

impl RepoHandle {
    pub(in crate::handle) fn read_keyed<T: KnownFSReadable + KeyedFilePath>(
        &self,
        key: &str,
    ) -> anyhow::Result<T> {
        let path = T::file_path(key).with_base_path(&self.repo_path);
        T::read_from_known(path).context(anyhow!("reading {:?}", T::file_name()))
    }
}
