use crate::application::repo::client::ClientRepoHandle;
use crate::domain::pack::pack_diff::PackDiff;
use crate::infrastructure::progress::progress_reporter::ProgressReporter;

impl ClientRepoHandle {
    pub async fn apply_pack_and_parents_diffs(
        &self,
        progress_reporter: impl ProgressReporter,
        diffs: Vec<PackDiff>,
    ) -> anyhow::Result<()> {
        for diff in diffs {
            let pack_name = diff.get_pack_name().to_string();
            self.apply_pack_diff(&pack_name, progress_reporter.clone(), diff)
                .await?;
        }
        Ok(())
    }
}
