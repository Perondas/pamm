pub mod get_canonical_addon_paths;
#[cfg(target_os = "linux")]
pub mod get_linux_addon_paths;
pub mod get_pack;
pub mod get_pack_index;
pub mod get_repo_info;

use crate::handle::client::client_repo_handle::ClientRepoHandle;
use crate::io::fs::fs_readable::{KnownFSReadable, NamedFSReadable};

impl ClientRepoHandle {
    #[allow(dead_code)]
    pub(in crate::handle) fn read<T: KnownFSReadable>(&self) -> anyhow::Result<T> {
        T::read_from_known(&self.repo_path)
    }

    pub(in crate::handle) fn read_named<T: NamedFSReadable>(
        &self,
        identifier: &str,
    ) -> anyhow::Result<T> {
        T::read_from_named(&self.repo_path, identifier)
    }
}
