use pamm_lib::pack::pack_diff::PackDiff;
use crate::api::commands::sync_pack::get_diff::{DiffResult, OpaqueDiff};
use crate::api::progress_reporting::DartProgressReporter;

pub fn sync_pack(
    pack_name: String,
    repo_path: String,
    dart_progress_reporter: &DartProgressReporter,
    pack_diff: OpaqueDiff
) -> anyhow::Result<()> {
    todo!()
}