pub mod commands;
pub mod args;
use anyhow::Result;
use clap::Parser;
use crate::args::{AppSubcommand, Args};
use crate::commands::init_pack::init_pack_command;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        AppSubcommand::InitPack(args) => init_pack_command(args)
    }
}
