use crate::api::commands::pack_sync::get_diff::{DiffResult, diff_to_result};
use crate::api::progress_reporting::DartProgressReporter;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use std::path::Path;

pub fn get_diff_with_parents(
    pack_name: String,
    repo_path: String,
    dart_progress_reporter: &DartProgressReporter,
    clear_cache: bool,
) -> anyhow::Result<MultiDiffResult> {
    let repo_dir = Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_dir)?;

    let diffs = handle.get_pack_and_parents_diffs(
        &pack_name,
        dart_progress_reporter.clone(),
        clear_cache,
    )?;

    let results: Vec<DiffResult> = diffs.into_iter().map(diff_to_result).collect();

    let has_changes = results.iter().any(|r| r.has_changes);
    let total_dl_size = results.iter().map(|r| r.total_dl_size).sum();
    let total_size_change = results.iter().map(|r| r.total_size_change).sum();
    let total_change_count = results.iter().map(|r| r.change_count).sum();
    let changed_packs = results.iter().filter(|r| r.has_changes).count();

    Ok(MultiDiffResult {
        results,
        has_changes,
        total_dl_size,
        total_size_change,
        total_change_count,
        changed_packs,
    })
}

pub struct MultiDiffResult {
    pub results: Vec<DiffResult>,
    pub has_changes: bool,
    pub total_dl_size: u64,
    pub total_size_change: i64,
    pub total_change_count: usize,
    pub changed_packs: usize,
}
