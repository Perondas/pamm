use clap::Args;
use pamm_lib::handle::repo_handle::RepoHandle;
use std::env::current_dir;

#[derive(Debug, Args)]
pub struct LaunchArgs {
    #[arg()]
    pub name: String,
}

pub fn launch_command(args: LaunchArgs) -> anyhow::Result<()> {
    let handle = RepoHandle::open(&current_dir()?)?;

    handle.launch_via_steam(&args.name)
}
