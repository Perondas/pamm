pub mod get_canonical_addon_paths;
#[cfg(target_os = "linux")]
pub mod get_linux_addon_paths;
pub mod get_pack;
pub mod get_repo_info;

use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::fs_readable::{KnownFSReadable, NamedFSReadable};
use crate::io::files::file_paths::rel_path::RelPath;
use anyhow::{anyhow, Context};

impl RepoHandle {
    pub(in crate::handle) fn read_named<T: NamedFSReadable>(
        &self,
        rel_path: &RelPath,
        identifier: &str,
    ) -> anyhow::Result<T> {
        let path = rel_path
            .with_base_path(&self.repo_path)
            .join(T::file_name(identifier));
        T::read_from_path(path).context(anyhow!("reading {:?}", identifier))
    }

    pub(in crate::handle) fn read_known<T: KnownFSReadable>(
        &self,
        rel_path: &RelPath,
    ) -> anyhow::Result<T> {
        let path = rel_path
            .with_base_path(&self.repo_path)
            .join(T::file_name());
        T::read_from_path(path).context(anyhow!("reading {:?}", T::file_name()))
    }
}
