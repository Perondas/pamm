pub mod args;
pub mod commands;
pub mod utils;
pub mod progress_reporting;

use crate::args::{AppSubcommand, Args};
use crate::commands::add_pack::add_pack_command;
use crate::commands::init_pack::init_repo_command;
use crate::commands::init_remote::init_remote_command;
use crate::commands::launch::launch_command;
use crate::commands::sync_pack::sync_pack_command;
use crate::commands::update_pack::update_pack_command;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        AppSubcommand::Init => init_repo_command(),
        AppSubcommand::AddPack(args) => add_pack_command(args),
        AppSubcommand::Update(args) => update_pack_command(args),
        AppSubcommand::InitRemote(args) => init_remote_command(args),
        AppSubcommand::Sync(args) => sync_pack_command(args),
        AppSubcommand::Launch(args) => launch_command(args),
    }
}
