pub mod get_canonical_addon_paths;
#[cfg(target_os = "linux")]
pub mod get_linux_addon_paths;
pub mod get_pack;
pub mod get_repo_info;

use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::fs_readable::NamedFSReadable;
use anyhow::{Context, anyhow};

impl RepoHandle {
    pub(in crate::handle) fn read_named<T: NamedFSReadable>(
        &self,
        identifier: &str,
    ) -> anyhow::Result<T> {
        let path = self.repo_path.join(T::get_rel_path(identifier));
        T::read_from_path(path).context(anyhow!("reading {:?}", identifier))
    }
}
