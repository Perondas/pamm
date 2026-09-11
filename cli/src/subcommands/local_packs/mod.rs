use crate::subcommands::local_packs::add_local_pack::AddLocalPackArgs;
use crate::subcommands::local_packs::remove_local_pack::RemoveLocalPackArgs;
use clap::Subcommand;

pub mod add_local_pack;
pub mod remove_local_pack;

#[derive(Debug, clap::Args)]
pub struct LocalPacksArgs {
    #[clap(subcommand)]
    pub command: LocalPacksSubcommand,
}

#[derive(Debug, Subcommand)]
/// Manage local packs
pub enum LocalPacksSubcommand {
    Add(AddLocalPackArgs),
    Remove(RemoveLocalPackArgs),
}
