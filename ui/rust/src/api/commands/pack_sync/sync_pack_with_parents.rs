use crate::api::commands::pack_sync::cancellation::{
    forget_checksum_index, DownloadCancelToken, DownloadOutcome,
};
use crate::api::commands::pack_sync::get_diff::OpaqueDiff;
use crate::api::progress_reporting::DartProgressReporter;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::io::progress_reporting::progress_reporter::ProgressReporter;
use std::path::Path;

pub async fn sync_pack_with_parents(
    repo_path: String,
    dart_progress_reporter: DartProgressReporter,
    diffs: Vec<OpaqueDiff>,
    cancel_token: &DownloadCancelToken,
) -> anyhow::Result<DownloadOutcome> {
    let inner: Vec<_> = diffs.into_iter().map(|d| d.0).collect();

    if inner.iter().all(|d| !d.has_changes()) {
        return Ok(DownloadOutcome::Completed);
    }

    let pack_names: Vec<String> = inner
        .iter()
        .map(|d| d.get_pack_name().to_string())
        .collect();

    let repo_dir = Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_dir).await?;

    // Cancelling drops the download future, which stops every in-flight
    // request and closes its files at the next await point.
    let applied = cancel_token
        .token()
        .run_until_cancelled(
            handle.apply_pack_and_parents_diffs(dart_progress_reporter.clone(), inner),
        )
        .await;

    match applied {
        Some(result) => {
            result?;
            Ok(DownloadOutcome::Completed)
        }
        None => {
            // The packs are applied one after another and it is not known how
            // far this got, so every pack in the chain counts as unfinished.
            for pack_name in &pack_names {
                forget_checksum_index(repo_dir, pack_name).await;
            }
            dart_progress_reporter.report_message("Download cancelled");
            dart_progress_reporter.finish();
            Ok(DownloadOutcome::Cancelled)
        }
    }
}
