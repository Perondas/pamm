use crate::subcommands::externals::remove_external::RemoveExternalArgs;
use add_external::AddExternalArgs;
use clap::Subcommand;
use pamm_lib::handle::externals::external_addon::ExternalAddon;
use std::path::Path;
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

fn externals_to_name(externals: &[ExternalAddon]) -> Vec<String> {
    externals
        .iter()
        .map(|e| {
            e.name.clone().unwrap_or(
                Path::new(&e.path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
            )
        })
        .collect()
}
