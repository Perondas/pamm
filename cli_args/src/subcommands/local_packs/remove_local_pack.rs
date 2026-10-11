use clap::Args;

/// Remove a local pack
#[derive(Debug, Args)]
pub struct RemoveLocalPackArgs {
    /// Pack name
    pub name: String,
}
