pub mod get_canonical_addon_paths;
pub mod get_pack;
pub mod get_repo_info;

use crate::application::repo::handle::RepoBase;
use crate::infrastructure::fs::files::file_paths::keyed_path::KeyedFilePath;
use crate::infrastructure::fs::files::fs_readable::KnownFSReadable;
use anyhow::{Context, anyhow};

impl RepoBase {
    pub(in crate::application) async fn read_keyed<T: KnownFSReadable + KeyedFilePath>(
        &self,
        key: &str,
    ) -> anyhow::Result<T> {
        let path = T::file_path(key).with_base_path(&self.repo_path);
        T::read_from_known(path)
            .await
            .context(anyhow!("reading {:?}", T::file_name()))
    }
}
