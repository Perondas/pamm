use clap::Args;

/// Add an external addon to a pack
#[derive(Debug, Args)]
pub struct AddExternalArgs {
    /// Pack name
    #[arg()]
    pub name: String,

    /// Path to the addon folder on disk
    #[arg()]
    pub path: String,
}
