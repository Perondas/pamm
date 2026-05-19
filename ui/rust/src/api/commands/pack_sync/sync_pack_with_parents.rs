use crate::api::commands::pack_sync::get_diff::OpaqueDiff;
use crate::api::progress_reporting::DartProgressReporter;
use pamm_lib::handle::repo_handle::RepoHandle;
use std::path::Path;

pub fn sync_pack_with_parents(
    repo_path: String,
    dart_progress_reporter: DartProgressReporter,
    diffs: Vec<OpaqueDiff>,
) -> anyhow::Result<()> {
    let inner: Vec<_> = diffs.into_iter().map(|d| d.0).collect();

    if inner.iter().all(|d| !d.has_changes()) {
        return Ok(());
    }

    let repo_dir = Path::new(&repo_path);

    let handle = RepoHandle::open(repo_dir)?;

    handle.apply_pack_and_parents_diffs(dart_progress_reporter, inner)
}
