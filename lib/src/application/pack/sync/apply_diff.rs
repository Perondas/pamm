use crate::application::pack::read::get_pack::GetPack;
use crate::application::repo::client::ClientRepoHandle;
use crate::domain::pack::pack_diff::PackDiff;
use crate::infrastructure::progress::progress_reporter::ProgressReporter;
use anyhow::ensure;

impl ClientRepoHandle {
    pub async fn apply_pack_diff(
        &self,
        pack_name: &str,
        progress_reporter: impl ProgressReporter,
        diff: PackDiff,
    ) -> anyhow::Result<()> {
        ensure!(
            diff.target_index.pack_name == pack_name,
            "Diff pack name '{}' does not match target pack name '{}'",
            diff.target_index.pack_name,
            pack_name
        );

        let (config, _) = self.get_pack_with_settings(pack_name).await?;

        diff.write_checksum_index_to_fs(&self.repo_path).await?;

        let diff_applier = config.diff_applier(self, self.get_remote_url(), progress_reporter);

        diff_applier.apply(diff).await
    }
}
