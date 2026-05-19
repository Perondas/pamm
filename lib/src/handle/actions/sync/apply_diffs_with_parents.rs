use crate::handle::repo_handle::RepoHandle;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::models::pack::pack_diff::PackDiff;

impl RepoHandle {
    pub fn apply_pack_and_parents_diffs<P: ProgressReporter>(
        &self,
        progress_reporter: P,
        diffs: Vec<PackDiff>,
    ) -> anyhow::Result<()> {
        for diff in diffs {
            let pack_name = diff.get_pack_name().to_string();
            self.apply_pack_diff(&pack_name, progress_reporter.clone(), diff)?;
        }
        Ok(())
    }
}
