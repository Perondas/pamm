use crate::api::commands::pack_sync::cancellation::{
    forget_checksum_index, DownloadCancelToken, DownloadOutcome,
};
use crate::api::commands::pack_sync::get_diff::OpaqueDiff;
use crate::api::progress_reporting::DartProgressReporter;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::io::progress_reporting::progress_reporter::ProgressReporter;
use std::path::Path;

pub async fn sync_pack(
    pack_name: String,
    repo_path: String,
    dart_progress_reporter: DartProgressReporter,
    diff: OpaqueDiff,
    cancel_token: &DownloadCancelToken,
) -> anyhow::Result<DownloadOutcome> {
    let diff = diff.0;

    if !diff.has_changes() {
        return Ok(DownloadOutcome::Completed);
    }

    let repo_dir = Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_dir).await?;

    // Cancelling drops the download future, which stops every in-flight
    // request and closes its files at the next await point.
    let applied = cancel_token
        .token()
        .run_until_cancelled(handle.apply_pack_diff(
            &pack_name,
            dart_progress_reporter.clone(),
            diff,
        ))
        .await;

    let outcome = match applied {
        Some(result) => {
            result?;
            DownloadOutcome::Completed
        }
        None => {
            forget_checksum_index(repo_dir, &pack_name).await;
            dart_progress_reporter.report_message("Download cancelled");
            DownloadOutcome::Cancelled
        }
    };

    dart_progress_reporter.finish();

    Ok(outcome)
}
