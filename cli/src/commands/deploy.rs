use clap::Args;
use pamm_lib::handle::server_repo_handle::ServerRepoHandle;
use std::env::current_dir;

#[derive(Debug, Args)]
pub struct DeployArgs {
    /// Pack name
    #[arg()]
    pub name: String,
}

pub fn deploy_command(args: DeployArgs) -> anyhow::Result<()> {
    let handle = ServerRepoHandle::open(&current_dir()?)?;

    handle.deploy_pack(&args.name)?;

    Ok(())
}
