use crate::subcommands::pack::add_pack::AddPackArgs;
use crate::subcommands::pack::clear_pack::ClearPackArgs;
use clap::Subcommand;

pub mod add_pack;
pub mod clear_pack;

#[derive(Debug, clap::Args)]
pub struct PackArgs {
    #[clap(subcommand)]
    pub command: PackSubcommand,
}

#[derive(Debug, Subcommand)]
/// Manage local packs
pub enum PackSubcommand {
    Add(AddPackArgs),
    Clear(ClearPackArgs),
}
