use add_external::AddExternalArgs;
use clap::Subcommand;
use remove_external::RemoveExternalArgs;
use toggle_externals::ToggleExternalsArgs;

pub mod add_external;
pub mod remove_external;
pub mod toggle_externals;

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
    Remove(RemoveExternalArgs),
}
