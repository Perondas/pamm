use crate::api::frb;
use crate::api::commands::sync_pack::file_change::{get_file_changes, FileChange};
use crate::api::progress_reporting::DartProgressReporter;
use pamm_lib::index::get_size::GetSize;
use pamm_lib::io::fs::fs_readable::{KnownFSReadable, NamedFSReadable};
use pamm_lib::io::fs::pack::index_generator::IndexGenerator;
use pamm_lib::io::net::downloadable::NamedDownloadable;
use pamm_lib::pack::pack_config::PackConfig;
use pamm_lib::pack::pack_diff::{diff_packs, PackDiff};
use pamm_lib::pack::pack_user_settings::PackUserSettings;
use pamm_lib::repo::repo_config::RepoConfig;
use pamm_lib::repo::repo_user_settings::RepoUserSettings;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use pamm_lib::index::node_diff::NodeDiff;
use crate::frb_generated::RustAutoOpaque;

pub fn get_diff(
    pack_name: String,
    repo_path: String,
    dart_progress_reporter: &DartProgressReporter,
    clear_cache: bool,
) -> anyhow::Result<DiffResult> {
    let current_dir = Path::new(&repo_path);

    let repo_user_settings = RepoUserSettings::read_from_known(current_dir)?
        .expect("No remote config found in current directory");

    let repo_config = RepoConfig::read_from_known(current_dir)?
        .expect("No repo config found in current directory");

    if !repo_config.packs.contains(&pack_name) {
        return Err(anyhow::anyhow!(
            "Pack '{}' is not part of the repository",
            pack_name
        ));
    }

    let local_pack_config = PackConfig::read_from_named(current_dir, &pack_name)?.ok_or(
        anyhow::anyhow!("Pack config for '{}' not found locally", pack_name),
    )?;

    let user_settings = PackUserSettings::read_from_named(current_dir, &pack_name)?.ok_or(
        anyhow::anyhow!("Pack user settings for '{}' not found locally", pack_name),
    )?;

    let index_generator = IndexGenerator::from_config(
        &local_pack_config,
        current_dir,
        dart_progress_reporter.clone(),
    )?;

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
        .0
        .iter()
        .map(|diff| {
            let name = diff.get_name().to_owned();
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