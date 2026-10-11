use clap::Args;

/// Interactively enable or disable optional addons of a pack
#[derive(Debug, Args)]
pub struct ToggleOptionalsArgs {
    /// Pack name
    #[arg()]
    pub name: String,
}
