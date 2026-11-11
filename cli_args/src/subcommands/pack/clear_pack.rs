use clap::Args;

/// Delete all the pack content from disk
#[derive(Debug, Args)]
pub struct ClearPackArgs {
    pub name: String,
}
