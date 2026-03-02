use crate::log_wrapper::LogWrapper;
use crate::progress_reporting::IndicatifProgressReporter;
use crate::utils::diff_to_string::ToPrettyString;
use clap::Args;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::handle::repo_handle::RepoHandle;
use pamm_lib::io::fs::pack::index_generator::IndexGenerator;
use pamm_lib::models::pack::pack_diff::diff_packs;
use std::env::current_dir;

#[derive(Debug, Args)]
pub struct UpdatePackArgs {
    #[arg()]
    pub name: String,

    /// Force refresh all addons, ignoring cached state
    #[arg(short, long, action)]
    pub force_refresh: bool,

    /// Silent mode, minimal output
    #[arg(short, long, action)]
    pub silent: bool,
}

pub fn update_pack_command(args: UpdatePackArgs, log_wrapper: LogWrapper) -> anyhow::Result<()> {
    let handle = RepoHandle::open(&current_dir()?)?;

    let stored_index = handle.get_pack_index(&args.name)?;

    let progress_reporter = if args.silent {
        IndicatifProgressReporter::disabled(log_wrapper)
    } else {
        IndicatifProgressReporter::new(log_wrapper)
    };

    let index_generator = IndexGenerator::from_handle(&handle, &args.name, progress_reporter)?;

    if args.force_refresh {
        index_generator.clear_cache()?;
    }

    let actual_index = index_generator.index_addons()?;

    let diff = diff_packs(stored_index, actual_index.clone())?;

    if !diff.has_changes() {
        println!("No changes found");
        return Ok(());
    }

    println!("{}", diff.to_pretty_string());

    let outcome = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to apply these changes?")
        .default(false)
        .interact()?;

    if !outcome {
        println!("Aborting update.");
        return Ok(());
    }

    handle.apply_diff(&diff)?;

    Ok(())
}
