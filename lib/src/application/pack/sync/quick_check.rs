use crate::application::pack::read::get_pack::GetPack;
use crate::application::repo::client::ClientRepoHandle;
use crate::domain::index::checksum_index::ChecksumIndex;
use crate::infrastructure::fs::files::file_paths::rel_path::RelPath;
use crate::infrastructure::fs::files::fs_readable::KnownFSReadable;
use crate::infrastructure::fs::files::name_consts::INDEX_DIR_NAME;
use crate::infrastructure::http::downloadable::KnownDownloadable;
use crate::infrastructure::http::remote_version::verify_remote_version;
use anyhow::{Context, anyhow};
use log::{debug, trace};
use std::collections::HashSet;

impl ClientRepoHandle {
    pub async fn quick_check_pack_up_to_date(&self, pack_name: &str) -> anyhow::Result<bool> {
        self.ensure_not_local(pack_name)?;

        let index_rel = RelPath::new().push(pack_name).push(INDEX_DIR_NAME);

        let remote_url = self.remote().clone();

        verify_remote_version(&remote_url).await?;

        let index_url = index_rel.with_base_url(&remote_url);

        debug!(
            "Performing quick check for pack '{}'. Remote index URL: {}",
            pack_name, index_url
        );

        let remote_repo_config = ChecksumIndex::download_known(&index_url)
            .await
            .context(anyhow!("Failed to download checksum index"))?;

        let index_dir = index_rel.with_base_path(&self.repo_path);

        let local_checksum_index = match ChecksumIndex::read_from_known(&index_dir).await {
            Ok(index) => index,
            Err(_) => {
                debug!(
                    "Local checksum index for pack '{}' not found or unreadable. Assuming not up to date.",
                    pack_name
                );
                return Ok(false);
            }
        };

        let (mut local_config, settings) = self.get_pack_with_settings(pack_name).await?;

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
