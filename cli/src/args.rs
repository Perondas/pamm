use crate::commands::update_pack::UpdatePackArgs;
use clap::{Parser, Subcommand};
use crate::commands::init_remote::InitRemoteArgs;

/// Pbo signing utility
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: AppSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AppSubcommand {
    Init,
    Update(UpdatePackArgs),
    InitRemote(InitRemoteArgs),
}
