use crate::api::commands::pack_sync::get_diff::OpaqueDiff;
use crate::api::progress_reporting::DartProgressReporter;
use pamm_lib::handle::repo_handle::RepoHandle;
use std::path::Path;

pub fn sync_pack(
    pack_name: String,
    repo_path: String,
    dart_progress_reporter: DartProgressReporter,
    pack_diff: OpaqueDiff,
) -> anyhow::Result<()> {
    let diff = pack_diff.0;

    if !diff.has_changes() {
        return Ok(());
    }

    let current_dir = Path::new(&repo_path);

    let handle = RepoHandle::open(current_dir)?;

    let repo_user_settings = handle.get_repo_user_settings()?;
    let (local_pack_config, _) = handle.get_pack_with_settings(&pack_name)?;

    let diff_applier = local_pack_config.diff_applier(
        &handle,
        repo_user_settings.get_remote(),
        dart_progress_reporter,
    );

    diff_applier.apply(diff)?;

    Ok(())
}
