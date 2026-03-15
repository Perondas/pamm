use crate::commands::optionals::toggle_optionals::ToggleOptionalsArgs;
use clap::Subcommand;

#[derive(Debug, clap::Args)]
pub struct OptionalArgs {
    #[clap(subcommand)]
    pub command: OptionalsSubcommand,
}

#[derive(Debug, Subcommand)]
/// Manage optional addons
pub enum OptionalsSubcommand {
    Toggle(ToggleOptionalsArgs),
}
