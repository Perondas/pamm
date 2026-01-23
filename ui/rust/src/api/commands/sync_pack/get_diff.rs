use crate::api::commands::sync_pack::diff_to_string::ToPrettyString;
use crate::api::progress_reporting::DartProgressReporter;
use pamm_lib::io::fs::fs_readable::{KnownFSReadable, NamedFSReadable};
use pamm_lib::io::fs::pack::index_generator::IndexGenerator;
use pamm_lib::io::net::downloadable::NamedDownloadable;
use pamm_lib::pack::pack_config::PackConfig;
use pamm_lib::pack::pack_diff::diff_packs;
use pamm_lib::pack::pack_user_settings::PackUserSettings;
use pamm_lib::repo::repo_config::RepoConfig;
use pamm_lib::repo::repo_user_settings::RepoUserSettings;
use std::path::Path;

pub fn get_diff(
    pack_name: String,
    repo_path: String,
    dart_progress_reporter: &DartProgressReporter,
) -> anyhow::Result<String> {
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
        &current_dir,
        dart_progress_reporter.clone(),
    )?;

    let actual_index = index_generator.index_addons()?;

    let mut remote_pack_config =
        PackConfig::download_named(repo_user_settings.get_remote(), &pack_name)?;

    remote_pack_config.remove_disabled_optionals(&user_settings);

    let remote_index = remote_pack_config.download_indexes(repo_user_settings.get_remote())?;

    let diff = diff_packs(actual_index, remote_index.clone())?;

    Ok(diff.to_pretty_string())
}
