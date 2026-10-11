use clap::Args;
use url::Url;

/// Initialize a new client repository from a remote repository
#[derive(Debug, Args)]
pub struct InitRemoteArgs {
    /// The URL of the remote repository to initialize from
    #[arg()]
    pub remote: Url,
}
