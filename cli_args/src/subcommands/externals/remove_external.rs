use clap::Args;

/// Interactively remove an external addon from a pack
#[derive(Debug, Args)]
pub struct RemoveExternalArgs {
    /// Pack name
    #[arg()]
    pub name: String,
}
