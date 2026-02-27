use crate::api::commands::pack_sync::get_diff::OpaqueDiff;
use crate::api::progress_reporting::DartProgressReporter;
use anyhow::{anyhow, Context};
use pamm_lib::io::fs::fs_readable::{KnownFSReadable, NamedFSReadable};
use pamm_lib::models::pack::pack_config::PackConfig;
use pamm_lib::models::repo::repo_user_settings::RepoUserSettings;
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

    let local_pack_config = PackConfig::read_from_named(current_dir, &pack_name).context(
        anyhow::anyhow!("Pack config for '{}' not found locally", pack_name),
    )?;

    let repo_user_settings = RepoUserSettings::read_from_known(current_dir).context(anyhow!(
        "Repository user settings not found in: {:#?}",
        current_dir
    ))?;

    let diff_applier = local_pack_config.diff_applier(
        current_dir,
        repo_user_settings.get_remote(),
        dart_progress_reporter,
    );

    diff_applier.apply(diff)?;

    Ok(())
}
