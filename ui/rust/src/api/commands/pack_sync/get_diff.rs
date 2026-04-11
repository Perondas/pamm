use crate::api::commands::pack_sync::file_change::{get_file_changes, FileChange};
use crate::api::frb;
use crate::api::progress_reporting::DartProgressReporter;
use crate::frb_generated::RustAutoOpaque;
use pamm_lib::handle::repo_handle::RepoHandle;
use pamm_lib::models::identifiable::Identifiable;
use pamm_lib::models::index::get_dl_size::GetDlSize;
use pamm_lib::models::index::get_size_change::GetSizeChange;
use pamm_lib::models::pack::pack_diff::PackDiff;
use std::collections::HashMap;
use std::path::Path;

pub fn get_diff(
    pack_name: String,
    repo_path: String,
    dart_progress_reporter: &DartProgressReporter,
    clear_cache: bool,
) -> anyhow::Result<DiffResult> {
    let repo_dir = Path::new(&repo_path);

    let handle = RepoHandle::open(repo_dir)?;

    let diff = handle.get_pack_diff(&pack_name, dart_progress_reporter.clone(), clear_cache)?;
    
    let file_changes = diff
        .addon_diffs
        .iter()
        .map(|diff| {
            let name = diff.get_identifier().to_owned();
            let changes = get_file_changes(diff);
            (name, changes)
        })
        .collect();

    Ok(DiffResult {
        has_changes: diff.has_changes(),
        change_count: diff.change_count(),
        total_dl_size: diff.get_dl_size(),
        total_size_change: diff.get_size_change(),
        diff: RustAutoOpaque::new(OpaqueDiff(diff)),
        file_changes,
    })
}

pub struct DiffResult {
    pub diff: RustAutoOpaque<OpaqueDiff>,
    pub has_changes: bool,
    pub change_count: usize,
    pub total_dl_size: u64,
    pub total_size_change: i64,
    pub file_changes: HashMap<String, Vec<FileChange>>,
}

// Required for the generator to not fail to import PackDiff
#[frb(opaque)]
pub struct OpaqueDiff(pub(crate) PackDiff);
