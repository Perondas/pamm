use crate::api::commands::pack_sync::file_change::{get_file_changes, FileChange};
use crate::api::frb;
use crate::api::progress_reporting::DartProgressReporter;
use crate::frb_generated::RustAutoOpaque;
use pamm_lib::handle::repo_handle::RepoHandle;
use pamm_lib::io::fs::pack::index_generator::IndexGenerator;
use pamm_lib::io::net::downloadable::NamedDownloadable;
use pamm_lib::models::identifiable::Identifiable;
use pamm_lib::models::index::get_size::GetSize;
use pamm_lib::models::pack::pack_config::PackConfig;
use pamm_lib::models::pack::pack_diff::{diff_packs, PackDiff};
use std::collections::HashMap;
use std::path::Path;

pub fn get_diff(
    pack_name: String,
    repo_path: String,
    dart_progress_reporter: &DartProgressReporter,
    clear_cache: bool,
) -> anyhow::Result<DiffResult> {
    let current_dir = Path::new(&repo_path);

    let handle = RepoHandle::open(current_dir)?;

    let repo_user_settings = handle.get_repo_user_settings()?;

    let (_, user_settings) = handle.get_pack_with_settings(&pack_name)?;

    let index_generator =
        IndexGenerator::from_handle(&handle, &pack_name, dart_progress_reporter.clone())?;

    if clear_cache {
        index_generator.clear_cache()?;
    }

    let actual_index = index_generator.index_addons()?;

    let mut remote_pack_config =
        PackConfig::download_named(repo_user_settings.get_remote(), &pack_name)?;

    remote_pack_config.remove_disabled_optionals(&user_settings);

    let remote_index = remote_pack_config.download_indexes(repo_user_settings.get_remote())?;

    let diff = diff_packs(actual_index, remote_index.clone())?;

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
        total_change_size: diff.get_size(),
        diff: RustAutoOpaque::new(OpaqueDiff(diff)),
        file_changes,
    })
}

pub struct DiffResult {
    pub diff: RustAutoOpaque<OpaqueDiff>,
    pub has_changes: bool,
    pub change_count: usize,
    pub total_change_size: u64,
    pub file_changes: HashMap<String, Vec<FileChange>>,
}

// Required for the generator to not fail to import PackDiff
#[frb(opaque)]
pub struct OpaqueDiff(pub(crate) PackDiff);
