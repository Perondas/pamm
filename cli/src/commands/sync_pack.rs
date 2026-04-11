use crate::log_wrapper::LogWrapper;
use crate::progress_reporting::IndicatifProgressReporter;
use crate::utils::diff_to_string::ToPrettyString;
use clap::Args;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::handle::actions::sync::config_sync_interactor::ConfigSyncInteractor;
use pamm_lib::handle::repo_handle::RepoHandle;
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
    let mut repo_handle = RepoHandle::open(&current_dir()?)?;

    repo_handle.sync_repo_config(&DialogerInteractor)?;

    let progress_reporter = if args.silent {
        IndicatifProgressReporter::disabled(log_wrapper)
    } else {
        IndicatifProgressReporter::new(log_wrapper)
    };

    let diff =
        repo_handle.get_pack_diff(&args.name, progress_reporter.clone(), args.force_refresh)?;

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

    repo_handle.apply_pack_diff(&args.name, progress_reporter, diff)?;

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
