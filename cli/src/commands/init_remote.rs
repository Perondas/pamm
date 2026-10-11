use pamm_cli_args::commands::init_remote::InitRemoteArgs;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::reading::get_repo_info::GetRepoInfo;

pub async fn init_remote_command(args: InitRemoteArgs) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    let handle = ClientRepoHandle::init_from_remote(&current_dir, &args.remote).await?;

    println!("{}", handle.get_config().to_pretty_string());

    Ok(())
}
