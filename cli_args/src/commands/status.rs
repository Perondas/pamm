use clap::Args;

/// Show the status of the local repository, or details about a single pack
#[derive(Debug, Args)]
pub struct StatusArgs {
    /// Name of a pack to show details for
    #[arg()]
    pub name: Option<String>,
}
