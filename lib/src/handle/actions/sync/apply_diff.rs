use crate::handle::reading::get_pack::GetPack;
use crate::handle::repo_handle::RepoHandle;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::models::pack::pack_diff::PackDiff;
use anyhow::ensure;

impl RepoHandle {
    pub fn apply_pack_diff<P: ProgressReporter>(
        &self,
        pack_name: &str,
        progress_reporter: P,
        diff: PackDiff,
    ) -> anyhow::Result<()> {
        ensure!(
            diff.target_index.pack_name == pack_name,
            "Diff pack name '{}' does not match target pack name '{}'",
            diff.target_index.pack_name,
            pack_name
        );

        let (config, _) = self.get_pack_with_settings(pack_name)?;

        let diff_applier = config.diff_applier(self, self.get_remote_url()?, progress_reporter);

        diff_applier.apply(diff)
    }
}
