use crate::commands::externals::add_external::AddExternalArgs;
use crate::commands::externals::toggle_externals::ToggleExternalsArgs;
use clap::Subcommand;

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
