use toggle_optionals::ToggleOptionalsArgs;
use clap::Subcommand;

pub mod toggle_optionals;

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
