use crate::commands::sync_pack::DialogerInteractor;
use crate::log_wrapper::LogWrapper;
use crate::progress_reporting::IndicatifProgressReporter;
use crate::utils::diff_to_string::ToPrettyString;
use crate::utils::workshop::report_workshop_addons_not_found;
use dialoguer::theme::ColorfulTheme;
use pamm_cli_args::commands::sync_this_only_pack::SyncThisOnlyPackArgs;
use pamm_lib::application::repo::client::ClientRepoHandle;
use std::env::current_dir;

pub async fn sync_this_pack_only_command(
    args: SyncThisOnlyPackArgs,
    log_wrapper: LogWrapper,
) -> anyhow::Result<()> {
    let mut repo_handle = ClientRepoHandle::open(&current_dir()?).await?;

    repo_handle.sync_repo_config(&DialogerInteractor).await?;

    let progress_reporter = if args.silent {
        IndicatifProgressReporter::disabled(log_wrapper)
    } else {
        IndicatifProgressReporter::new(log_wrapper)
    };

    let diff = repo_handle
        .get_pack_diff(&args.name, progress_reporter.clone(), args.force_refresh)
        .await?;

    if !diff.has_changes() {
        println!("No changes found.");
        return report_workshop_addons_not_found(&repo_handle, &args.name).await;
    }

    println!("{}", diff.to_pretty_string());

    let outcome = args.yes
        || dialoguer::Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to download these changes?")
            .default(false)
            .interact()?;

    if !outcome {
        println!("Aborting sync.");
        return Ok(());
    }

    repo_handle
        .apply_pack_diff(&args.name, progress_reporter, diff)
        .await?;

    println!("Pack synchronized successfully.");

    report_workshop_addons_not_found(&repo_handle, &args.name).await
}
