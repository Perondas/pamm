use clap::Args;

/// Deploys a chosen pack to the server
#[derive(Debug, Args)]
pub struct DeployArgs {
    /// Pack name
    #[arg()]
    pub name: String,
}
