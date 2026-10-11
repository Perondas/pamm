use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::handle::reading::get_pack::GetPack;
use crate::io::fs::pack::index_generator::IndexGenerator;
use crate::io::net::remote_version::verify_remote_version;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_diff::{PackDiff, diff_packs};

impl ClientRepoHandle {
    pub async fn get_pack_diff(
        &self,
        pack_name: &str,
        progress_reporter: impl ProgressReporter,
        force_refresh: bool,
    ) -> anyhow::Result<PackDiff> {
        self.ensure_not_local(pack_name)?;

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
