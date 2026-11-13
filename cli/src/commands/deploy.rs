use pamm_cli_args::commands::deploy::DeployArgs;
use pamm_lib::handle::repo_handle::RepoHandle;
use pamm_lib::handle::server_repo_handle::ServerRepoHandle;
use std::env::current_dir;

pub async fn deploy_command(args: DeployArgs) -> anyhow::Result<()> {
    let handle: ServerRepoHandle = RepoHandle::open(&current_dir()?).await?.try_into()?;

    handle.deploy_pack(&args.name).await?;

    Ok(())
}
