use clap::Args;

/// Interactively enable or disable external addons of a pack
#[derive(Debug, Args)]
pub struct ToggleExternalsArgs {
    /// Pack name
    #[arg()]
    pub name: String,
}
