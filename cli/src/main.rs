pub mod commands;
mod log_wrapper;
pub mod progress_reporting;
pub mod subcommands;
pub mod utils;

use crate::commands::add_pack::add_pack_command;
use crate::commands::build::build_command;
use crate::commands::deploy::deploy_command;
use crate::commands::init_remote::init_remote_command;
use crate::commands::init_repo::init_repo_command;
use crate::commands::launch::launch_command;
use crate::commands::status::status_command;
use crate::commands::sync_pack::sync_pack_command;
use crate::commands::sync_this_only_pack::sync_this_only_pack_command;
use crate::subcommands::externals::remove_external::remove_external_command;
use crate::subcommands::local_packs::remove_local_pack::remove_local_pack_command;
use anyhow::Result;
use clap::Parser;
use log::{debug, trace};
use pamm_cli_args::args::{AppSubcommand, Args};
use pamm_cli_args::subcommands::externals::ExternalsSubcommand;
use pamm_cli_args::subcommands::local_packs::LocalPacksSubcommand;
use pamm_cli_args::subcommands::optionals::OptionalsSubcommand;
use subcommands::externals::add_external::add_external_command;
use subcommands::externals::toggle_externals::toggle_externals_command;
use subcommands::local_packs::add_local_pack::add_local_pack_command;
use subcommands::optionals::toggle_optionals::toggle_optionals_command;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let log_level = args.log_level.clone().unwrap_or("warn".to_string());

    let log_wrapper = log_wrapper::LogWrapper::new(
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level))
            .build(),
    );

    let log_wrapper = log_wrapper.try_init()?;

    trace!(
        "Running in: {:?}",
        std::env::current_dir().unwrap_or_default()
    );
    debug!("Got args: {:#?}", args);

    match args.command {
        AppSubcommand::Init => init_repo_command().await,
        AppSubcommand::AddPack(args) => add_pack_command(args).await,
        AppSubcommand::Build(args) => build_command(args, log_wrapper).await,
        AppSubcommand::InitRemote(args) => init_remote_command(args).await,
        AppSubcommand::Sync(args) => sync_pack_command(args, log_wrapper).await,
        AppSubcommand::SyncThisOnly(args) => sync_this_only_pack_command(args, log_wrapper).await,
        AppSubcommand::Launch(args) => launch_command(args).await,
        #[cfg(target_os = "linux")]
        AppSubcommand::Linux(args) => {
            use pamm_cli_args::subcommands::linux::LinuxSubcommand;
            use subcommands::linux::*;
            match args.command {
                None => status::status_command().await,
                Some(LinuxSubcommand::InstallTeamspeak(args)) => {
                    install_teamspeak::install_teamspeak_command(args)
                }
                Some(LinuxSubcommand::Teamspeak(args)) => {
                    launch_teamspeak::launch_teamspeak_command(args)
                }
                Some(LinuxSubcommand::Explorer(args)) => explorer::explorer_command(args),
                Some(LinuxSubcommand::Kill(args)) => kill::kill_command(args),
            }
        }
        AppSubcommand::Externals(args) => match args.command {
            ExternalsSubcommand::Toggle(args) => toggle_externals_command(args).await,
            ExternalsSubcommand::Add(args) => add_external_command(args).await,
            ExternalsSubcommand::Remove(args) => remove_external_command(args).await,
        },
        AppSubcommand::Optionals(args) => match args.command {
            OptionalsSubcommand::Toggle(args) => toggle_optionals_command(args).await,
        },
        AppSubcommand::Local(args) => match args.command {
            LocalPacksSubcommand::Add(args) => add_local_pack_command(args).await,
            LocalPacksSubcommand::Remove(args) => remove_local_pack_command(args).await,
        },
        AppSubcommand::Deploy(args) => deploy_command(args).await,
        AppSubcommand::Status(args) => status_command(args).await,
    }
}
