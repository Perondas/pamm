use crate::handle::client::reading::get_pack::GetPack;
use crate::handle::client::client_repo_handle::ClientRepoHandle;
use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::name_consts::{INDEX_DIR_NAME, get_pack_addon_directory_name};
use crate::io::net::downloadable::KnownDownloadable;
use crate::io::rel_path::RelPath;
use crate::models::index::checksum_index::ChecksumIndex;
use anyhow::{Context, anyhow};
use log::{debug, trace};
use std::collections::HashSet;

impl ClientRepoHandle {
    pub fn quick_check_pack_up_to_date(&self, pack_name: &str) -> anyhow::Result<bool> {
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

        let local_checksum_index = match ChecksumIndex::read_from_known(&index_dir) {
            Ok(index) => index,
            Err(_) => {
                debug!(
                    "Local checksum index for pack '{}' not found or unreadable. Assuming not up to date.",
                    pack_name
                );
                return Ok(false);
            }
        };

        let (mut local_config, settings) = self.get_pack_with_settings(pack_name)?;

        local_config.remove_disabled_optionals(&settings);

        let expected_addons: HashSet<_> = local_config.addons.keys().collect();
        let actual_addons: HashSet<_> = local_checksum_index.checksums.keys().collect();

        if expected_addons != actual_addons {
            debug!(
                "Local addons for pack '{}' do not match expected addons based on config/settings. Assuming not up to date.",
                pack_name
            );
            return Ok(false);
        }

        trace!(
            "Local checksum index for pack '{}': {:?}",
            pack_name, local_checksum_index
        );
        trace!(
            "Remote checksum index for pack '{}': {:?}",
            pack_name, remote_repo_config
        );

        let local_all_up_to_date = local_checksum_index
            .checksums
            .iter()
            .all(|(name, checksum)| remote_repo_config.checksums.get(name) == Some(checksum));

        Ok(local_all_up_to_date)
    }
}
