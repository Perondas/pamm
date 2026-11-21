use crate::application::pack::read::get_pack::GetPack;
use crate::application::repo::client::ClientRepoHandle;
use crate::domain::index::checksum_index::ChecksumIndex;
use crate::infrastructure::fs::files::file_paths::rel_path::RelPath;
use crate::infrastructure::fs::files::fs_readable::KnownFSReadable;
use crate::infrastructure::fs::files::name_consts::INDEX_DIR_NAME;
use crate::infrastructure::http::downloadable::KnownDownloadable;
use crate::infrastructure::http::remote_version::verify_remote_version;
use anyhow::{Context, anyhow, ensure};
use log::{debug, trace};
use std::collections::HashSet;

impl ClientRepoHandle {
    pub async fn quick_check_pack_up_to_date(&self, pack_name: &str) -> anyhow::Result<bool> {
        ensure!(
            !self.is_local_pack(pack_name),
            "Pack '{}' is a local pack and cannot be quick checked",
            pack_name
        );

        let index_rel = RelPath::new().push(pack_name).push(INDEX_DIR_NAME);

        let remote_url = self.get_remote_url().clone();

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

        let (local_config, settings) = self.get_pack_with_settings(pack_name).await?;
        let active_addons = local_config.active_addons(&settings).collect::<Vec<_>>();

        // An addon that cannot be found at all means the pack is definitely out
        // of sync. This is the only check workshop addons get.
        for addon in &active_addons {
            if addon.locate(&self.repo_path)?.is_none() {
                debug!(
                    "Addon {} not found. Pack '{}' is not up to date.",
                    addon, pack_name
                );
                return Ok(false);
            }
        }

        // Served addons are additionally checked against the checksum index,
        // which workshop addons never appear in.
        let expected_addons: HashSet<_> = active_addons
            .iter()
            .filter(|addon| !addon.settings.is_workshop())
            .map(|addon| &addon.name)
            .collect();
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
