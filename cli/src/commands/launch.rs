use clap::Args;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use std::env::current_dir;

#[derive(Debug, Args)]
pub struct LaunchArgs {
    #[arg()]
    pub name: String,
}

pub fn launch_command(args: LaunchArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&current_dir()?)?;

    handle.launch_via_steam(&args.name)
}
