use pamm_cli_args::commands::deploy::DeployArgs;
use pamm_lib::application::repo::server::ServerRepoHandle;
use std::env::current_dir;

pub async fn deploy_command(args: DeployArgs) -> anyhow::Result<()> {
    let handle = ServerRepoHandle::open(&current_dir()?).await?;

    handle.deploy_pack(&args.name).await?;

    Ok(())
}
