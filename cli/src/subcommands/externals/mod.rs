use add_external::AddExternalArgs;
use toggle_externals::ToggleExternalsArgs;
use clap::Subcommand;

pub mod toggle_externals;
pub mod add_external;

#[derive(Debug, clap::Args)]
pub struct ExternalsArgs {
    #[clap(subcommand)]
    pub command: ExternalsSubcommand,
}

#[derive(Debug, Subcommand)]
/// Manage external addons
pub enum ExternalsSubcommand {
    Toggle(ToggleExternalsArgs),
    Add(AddExternalArgs),
}
