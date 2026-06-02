pub mod get_canonical_addon_paths;
#[cfg(target_os = "linux")]
pub mod get_linux_addon_paths;
pub mod get_pack;
pub mod get_repo_info;

use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::fs_readable::NamedFSReadable;

impl RepoHandle {
    pub(in crate::handle) fn read_named<T: NamedFSReadable>(
        &self,
        identifier: &str,
    ) -> anyhow::Result<T> {
        T::read_from_named(&self.repo_path, identifier)
    }
}
