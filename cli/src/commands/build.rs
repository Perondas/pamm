use crate::log_wrapper::LogWrapper;
use crate::progress_reporting::IndicatifProgressReporter;
use clap::Args;
use pamm_lib::handle::actions::build::{BuildMode, BuildOptions};
use pamm_lib::handle::server_repo_handle::ServerRepoHandle;
use std::env::current_dir;

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Pack name; if omitted, builds the entire repo.
    #[arg()]
    pub name: Option<String>,

    /// Materialize files in `www/` as copies instead of symlinks.
    #[arg(long, action, conflicts_with = "symlink")]
    pub copy: bool,

    /// Materialize files in `www/` as symlinks (default).
    #[arg(long, action)]
    pub symlink: bool,

    /// Re-index everything from scratch, ignoring the cache.
    #[arg(short, long, action)]
    pub force_refresh: bool,

    /// Silent mode, minimal output.
    #[arg(short, long, action)]
    pub silent: bool,
}

pub fn build_command(args: BuildArgs, log_wrapper: LogWrapper) -> anyhow::Result<()> {
    let handle = ServerRepoHandle::open(&current_dir()?)?;

    let mode = if args.copy {
        BuildMode::Copy
    } else {
        BuildMode::Symlink
    };

    let progress_reporter = if args.silent {
        IndicatifProgressReporter::disabled(log_wrapper)
    } else {
        IndicatifProgressReporter::new(log_wrapper)
    };

    let opts = BuildOptions {
        mode,
        force_refresh: args.force_refresh,
    };

    match args.name {
        Some(pack) => {
            let report = handle.build_pack(&pack, opts, &progress_reporter)?;
            println!(
                "Built pack '{}' ({} files, mode {:?}).",
                pack, report.files_materialized, report.mode,
            );
        }
        None => {
            let report = handle.build(opts, progress_reporter)?;

            println!(
                "Built {} files, {} stale entries removed, mode {:?}.",
                report.files_materialized, report.stale_removed, report.mode,
            );
        }
    }

    Ok(())
}
