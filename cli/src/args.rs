use crate::commands::add_pack::AddPackArgs;
use crate::commands::init_remote::InitRemoteArgs;
use crate::commands::launch::LaunchArgs;
use crate::commands::sync_pack::SyncPackArgs;
use crate::commands::update_pack::UpdatePackArgs;
use clap::{Parser, Subcommand};

/// Personal ARMA mod manager CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: AppSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AppSubcommand {
    Init,
    InitRemote(InitRemoteArgs),
    AddPack(AddPackArgs),
    Update(UpdatePackArgs),
    Sync(SyncPackArgs),
    Launch(LaunchArgs),
}
