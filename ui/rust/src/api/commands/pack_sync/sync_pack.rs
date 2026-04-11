use crate::api::commands::pack_sync::get_diff::OpaqueDiff;
use crate::api::progress_reporting::DartProgressReporter;
use pamm_lib::handle::repo_handle::RepoHandle;
use std::path::Path;

pub fn sync_pack(
    pack_name: String,
    repo_path: String,
    dart_progress_reporter: DartProgressReporter,
    diff: OpaqueDiff,
) -> anyhow::Result<()> {
    let diff = diff.0;

    if !diff.has_changes() {
        return Ok(());
    }

    let repo_dir = Path::new(&repo_path);

    let handle = RepoHandle::open(repo_dir)?;

    handle.apply_pack_diff(&pack_name, dart_progress_reporter, diff)?;

    Ok(())
}
