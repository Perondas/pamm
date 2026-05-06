use crate::handle::reading::get_pack::GetPack;
use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::pack::index_generator::IndexGenerator;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_diff::{PackDiff, diff_packs};
use anyhow::Context;
use log::debug;

impl RepoHandle {
    pub fn get_pack_diff<P: ProgressReporter>(
        &self,
        pack_name: &str,
        progress_reporter: P,
        force_refresh: bool,
    ) -> anyhow::Result<Option<PackDiff>> {
        if !force_refresh {
            let qc_res = self
                .quick_check_pack_up_to_date(pack_name)
                .context("Failed to perform quick check")?;

            if qc_res {
                debug!(
                    "Quick check indicates pack '{}' is up to date. Skipping diff calculation.",
                    pack_name
                );
                return Ok(None);
            }
        }

        let (_, settings) = self.get_pack_with_settings(pack_name)?;

        let index_generator = IndexGenerator::from_handle(self, pack_name, progress_reporter)?;

        if force_refresh {
            index_generator.clear_cache()?;
        }

        let actual_index = index_generator.index_addons()?;

        let mut remote_pack_config: PackConfig = self.download_named(pack_name)?;

        remote_pack_config.remove_disabled_optionals(&settings);

        let remote_index = remote_pack_config.download_indexes(self.get_remote_url()?)?;

        actual_index.write_checksum_index_to_fs(&self.repo_path)?;

        Ok(Some(diff_packs(actual_index, remote_index)?))
    }
}
