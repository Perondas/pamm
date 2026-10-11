use pamm_cli_args::commands::deploy::DeployArgs;
use pamm_lib::handle::server_repo_handle::ServerRepoHandle;
use std::env::current_dir;

pub fn deploy_command(args: DeployArgs) -> anyhow::Result<()> {
    let handle = ServerRepoHandle::open(&current_dir()?)?;

    handle.deploy_pack(&args.name)?;

    Ok(())
}
