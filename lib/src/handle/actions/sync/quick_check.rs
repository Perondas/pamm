use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::name_consts::{get_pack_addon_directory_name, INDEX_DIR_NAME};
use crate::io::net::downloadable::KnownDownloadable;
use crate::io::rel_path::RelPath;
use crate::models::index::checksum_index::ChecksumIndex;
use anyhow::{anyhow, Context};
use log::debug;

impl RepoHandle {
    pub fn quick_check_pack_up_to_date(&mut self, pack_name: &str) -> anyhow::Result<bool> {
        let repo_user_settings = self
            .repo_user_settings
            .as_ref()
            .ok_or_else(|| anyhow!("Repo user settings not found"))?;

        let addon_path = RelPath::new().push(&get_pack_addon_directory_name(pack_name));

        let remote_url = repo_user_settings.get_remote().clone();

        let index_url = addon_path
            .clone()
            .push(INDEX_DIR_NAME)
            .with_base_url(&remote_url);

        debug!(
            "Performing quick check for pack '{}'. Remote index URL: {}",
            pack_name, index_url
        );

        let remote_repo_config = ChecksumIndex::download_known(&index_url)
            .context(anyhow!("Failed to download checksum index"))?;

        let index_dir = addon_path
            .clone()
            .push(INDEX_DIR_NAME)
            .with_base_path(&self.repo_path);

        let local_checksum_index = ChecksumIndex::read_from_known(&index_dir).unwrap_or_default();

        Ok(remote_repo_config.checksums == local_checksum_index.checksums)
    }
}
