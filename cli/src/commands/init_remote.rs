use clap::Args;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::reading::get_repo_info::GetRepoInfo;
use url::Url;

#[derive(Debug, Args)]
pub struct InitRemoteArgs {
    /// The URL of the remote repository to initialize from
    #[arg()]
    pub remote: Url,
}

pub fn init_remote_command(args: InitRemoteArgs) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    let handle = ClientRepoHandle::init_from_remote(&current_dir, &args.remote)?;

    println!("{}", handle.get_config().to_pretty_string());

    Ok(())
}
