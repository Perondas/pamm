use crate::log_wrapper::LogWrapper;
use crate::progress_reporting::IndicatifProgressReporter;
use crate::utils::diff_to_string::ToPrettyString;
use anyhow::{anyhow, Context};
use clap::Args;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::actions::sync::interactor::ConfigSyncInteractor;
use pamm_lib::actions::sync::sync_pack::sync_pack_config;
use pamm_lib::io::fs::fs_readable::{KnownFSReadable, NamedFSReadable};
use pamm_lib::io::fs::pack::index_generator::IndexGenerator;
use pamm_lib::io::net::downloadable::NamedDownloadable;
use pamm_lib::models::pack::pack_config::PackConfig;
use pamm_lib::models::pack::pack_diff::diff_packs;
use pamm_lib::models::pack::pack_user_settings::PackUserSettings;
use pamm_lib::models::repo::repo_user_settings::RepoUserSettings;
use std::env::current_dir;

#[derive(Debug, Args)]
pub struct SyncPackArgs {
    #[arg()]
    pub name: String,
    #[arg(short, long, default_value_t = false)]
    pub force_refresh: bool,
    /// Silent mode, minimal output
    #[arg(short, long, action)]
    pub silent: bool,
}

pub fn sync_pack_command(args: SyncPackArgs, log_wrapper: LogWrapper) -> anyhow::Result<()> {
    let current_dir = current_dir()?;

    let repo_user_settings = RepoUserSettings::read_from_known(&current_dir)
        .context("Could not find user settings in current directory")?;

    let repo_config = sync_pack_config(&current_dir, &DialogerInteractor)?;

    if !repo_config.packs.contains(&args.name) {
        return Err(anyhow::anyhow!(
            "Pack '{}' is not part of the repository",
            args.name
        ));
    }

    let local_pack_config = PackConfig::read_from_named(&current_dir, &args.name)
        .context(anyhow!("Pack config for '{}' not found locally", args.name))?;

    let user_settings = PackUserSettings::read_from_named(&current_dir, &args.name).context(
        anyhow!("Pack user settings for '{}' not found locally", args.name),
    )?;

    let progress_reporter = if args.silent {
        IndicatifProgressReporter::disabled(log_wrapper)
    } else {
        IndicatifProgressReporter::new(log_wrapper)
    };

    let index_generator =
        IndexGenerator::from_config(&local_pack_config, &current_dir, progress_reporter.clone())?;

    if args.force_refresh {
        index_generator.clear_cache()?;
    }

    let actual_index = index_generator.index_addons()?;

    let mut remote_pack_config =
        PackConfig::download_named(repo_user_settings.get_remote(), &args.name)?;

    remote_pack_config.remove_disabled_optionals(&user_settings);

    let remote_index = remote_pack_config.download_indexes(repo_user_settings.get_remote())?;

    let diff = diff_packs(actual_index, remote_index.clone())?;

    if !diff.has_changes() {
        println!("Pack is already up to date.");
        return Ok(());
    }

    println!("{}", diff.to_pretty_string());

    let outcome = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to download these changes?")
        .default(false)
        .interact()?;

    if !outcome {
        println!("Aborting sync.");
        return Ok(());
    }

    let diff_applier = local_pack_config.diff_applier(
        &current_dir,
        repo_user_settings.get_remote(),
        progress_reporter,
    );

    diff_applier.apply(diff)?;

    println!("Pack synchronized successfully.");

    Ok(())
}

struct DialogerInteractor;

impl ConfigSyncInteractor for DialogerInteractor {
    fn confirm_pack_removal(&self, pack_name: &str) -> anyhow::Result<bool> {
        let outcome = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Pack '{}' has been removed from remote repository. Do you want to remove all local files as well?",
                pack_name
            ))
            .default(false)
            .interact()?;
        Ok(outcome)
    }

    fn notify_pack_added(&self, pack_name: &str) -> anyhow::Result<()> {
        println!("Pack '{}' has been added to repository.", pack_name);
        Ok(())
    }
}
