use crate::log_wrapper::LogWrapper;
use crate::progress_reporting::IndicatifProgressReporter;
use crate::utils::diff_to_string::ToPrettyString;
use clap::Args;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use std::env::current_dir;
use crate::commands::sync_pack::DialogerInteractor;

#[derive(Debug, Args)]
pub struct SyncThisOnlyPackArgs {
    #[arg()]
    pub name: String,
    #[arg(short, long, default_value_t = false)]
    pub force_refresh: bool,
    /// Silent mode, minimal output
    #[arg(short, long, action)]
    pub silent: bool,
}

pub fn sync_this_only_pack_command(
    args: SyncThisOnlyPackArgs,
    log_wrapper: LogWrapper,
) -> anyhow::Result<()> {
    let mut repo_handle = ClientRepoHandle::open(&current_dir()?)?;

    repo_handle.sync_repo_config(&DialogerInteractor)?;

    let progress_reporter = if args.silent {
        IndicatifProgressReporter::disabled(log_wrapper)
    } else {
        IndicatifProgressReporter::new(log_wrapper)
    };

    let diff =
        repo_handle.get_pack_diff(&args.name, progress_reporter.clone(), args.force_refresh)?;

    if !diff.has_changes() {
        println!("No changes found.");
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
