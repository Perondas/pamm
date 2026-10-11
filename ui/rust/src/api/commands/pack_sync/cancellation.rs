use flutter_rust_bridge::frb;
use pamm_lib::io::files::file_names::fixed_file::FixedFile;
use pamm_lib::models::index::checksum_index::ChecksumIndex;
use std::path::Path;
use tokio_util::sync::CancellationToken;

/// Handed to a download so the Dart side can stop it.
#[frb(opaque)]
pub struct DownloadCancelToken(CancellationToken);

impl DownloadCancelToken {
    #[frb(sync)]
    pub fn cancel(&self) {
        self.0.cancel();
    }

    #[frb(ignore)]
    pub(crate) fn token(&self) -> CancellationToken {
        self.0.clone()
    }
}

#[frb(sync)]
pub fn create_download_cancel_token() -> DownloadCancelToken {
    DownloadCancelToken(CancellationToken::new())
}

pub enum DownloadOutcome {
    Completed,
    Cancelled,
}

/// Leaves a pack whose download was cancelled looking out of date.
///
/// Applying a diff records the *target* checksums before any file is
/// downloaded, so after a cancel the quick check would otherwise report the
/// pack as up to date. Without the index it reports it as stale, and the next
/// diff re-hashes the files on disk, which also catches anything half written.
#[frb(ignore)]
pub(crate) async fn forget_checksum_index(repo_path: &Path, pack_name: &str) {
    // Mirrors the lib's `<pack>/indexes/` layout (`INDEX_DIR_NAME` is crate-private).
    let path = repo_path
        .join(pack_name)
        .join("indexes")
        .join(ChecksumIndex::file_name());

    match tokio::fs::remove_file(&path).await {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => log::warn!("Failed to remove checksum index {path:?} after cancel: {e}"),
    }
}
