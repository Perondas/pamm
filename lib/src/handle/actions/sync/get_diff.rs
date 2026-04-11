use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::pack::index_generator::IndexGenerator;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_diff::{PackDiff, diff_packs};

impl RepoHandle {
    pub fn get_pack_diff<P: ProgressReporter>(
        &self,
        pack_name: &str,
        progress_reporter: P,
        force_refresh: bool,
    ) -> anyhow::Result<PackDiff> {
        let (_, settings) = self.get_pack_with_settings(pack_name)?;

        let index_generator = IndexGenerator::from_handle(self, pack_name, progress_reporter)?;

        if force_refresh {
            index_generator.clear_cache()?;
        }

        let actual_index = index_generator.index_addons()?;

        let mut remote_pack_config: PackConfig = self.download_named(pack_name)?;

        remote_pack_config.remove_disabled_optionals(&settings);

        let remote_index = remote_pack_config.download_indexes(self.get_remote_url()?)?;

        diff_packs(actual_index, remote_index)
    }
}
