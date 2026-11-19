use crate::application::pack::read::get_pack::GetPack;
use crate::application::repo::client::ClientRepoHandle;
use crate::domain::pack::pack_config::PackConfig;
use crate::domain::pack::pack_diff::{PackDiff, diff_packs};
use crate::infrastructure::fs::pack::index_generator::IndexGenerator;
use crate::infrastructure::http::remote_version::verify_remote_version;
use crate::infrastructure::progress::progress_reporter::ProgressReporter;
use anyhow::ensure;

impl ClientRepoHandle {
    pub async fn get_pack_diff(
        &self,
        pack_name: &str,
        progress_reporter: impl ProgressReporter,
        force_refresh: bool,
    ) -> anyhow::Result<PackDiff> {
        ensure!(
            !self.is_local_pack(pack_name),
            "Pack '{}' is a local pack and cannot be diffed against the remote",
            pack_name
        );

        verify_remote_version(self.get_remote_url()).await?;

        let (_, settings) = self.get_pack_with_settings(pack_name).await?;

        let index_generator =
            IndexGenerator::from_handle(self, pack_name, progress_reporter).await?;

        if force_refresh {
            index_generator.clear_cache().await?;
        }

        // Hashing the local files and fetching the remote indexes are
        // independent, so overlap them. Both run to completion even if one
        // fails: indexing can't be cancelled once on the blocking pool, and
        // must not be left holding the cache while the caller retries.
        let remote_index = async {
            let mut remote_pack_config: PackConfig = self.download_keyed(pack_name).await?;
            remote_pack_config.remove_disabled_optionals(&settings);
            remote_pack_config
                .download_indexes(self.get_remote_url())
                .await
        };

        let (actual_index, remote_index) =
            tokio::join!(index_generator.index_addons(), remote_index);
        let (actual_index, remote_index) = (actual_index?, remote_index?);

        actual_index
            .write_checksum_index_to_fs(&self.repo_path)
            .await?;

        diff_packs(actual_index, remote_index)
    }
}
