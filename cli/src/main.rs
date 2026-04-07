pub mod args;
pub mod commands;
mod log_wrapper;
pub mod progress_reporting;
pub mod subcommands;
pub mod utils;

use crate::args::{AppSubcommand, Args};
use crate::commands::add_pack::add_pack_command;
use subcommands::externals::add_external::add_external_command;
use crate::commands::init_remote::init_remote_command;
use crate::commands::init_repo::init_repo_command;
use crate::commands::launch::launch_command;
use subcommands::optionals::toggle_optionals::toggle_optionals_command;
use crate::commands::sync_pack::sync_pack_command;
use crate::commands::update_pack::update_pack_command;
use crate::subcommands::externals::ExternalsSubcommand;
use crate::subcommands::optionals::OptionalsSubcommand;
use anyhow::Result;
use clap::Parser;
use subcommands::externals::toggle_externals::toggle_externals_command;

fn main() -> Result<()> {
    let args = Args::parse();

    let log_level = args.log_level.unwrap_or("warn".to_string());

    let log_wrapper = log_wrapper::LogWrapper::new(
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level))
            .build(),
    );

    let log_wrapper = log_wrapper.try_init()?;

    match args.command {
        AppSubcommand::Init => init_repo_command(),
        AppSubcommand::AddPack(args) => add_pack_command(args),
        AppSubcommand::Update(args) => update_pack_command(args, log_wrapper),
        AppSubcommand::InitRemote(args) => init_remote_command(args),
        AppSubcommand::Sync(args) => sync_pack_command(args, log_wrapper),
        AppSubcommand::Launch(args) => launch_command(args),
        AppSubcommand::Externals(args) => match args.command {
            ExternalsSubcommand::Toggle(args) => toggle_externals_command(args),
            ExternalsSubcommand::Add(args) => add_external_command(args),
        },
        AppSubcommand::Optionals(args) => match args.command {
            OptionalsSubcommand::Toggle(args) => toggle_optionals_command(args),
        },
    }
}
