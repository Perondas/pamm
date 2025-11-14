pub mod args;
pub mod commands;
use crate::args::{AppSubcommand, Args};
use crate::commands::init_pack::init_pack_command;
use crate::commands::init_remote::init_remote_command;
use crate::commands::update_pack::update_pack_command;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        AppSubcommand::Init => init_pack_command(),
        AppSubcommand::Update(args) => update_pack_command(args),
        AppSubcommand::InitRemote(args) => init_remote_command(args),
    }
}
