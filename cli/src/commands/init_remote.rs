use pamm_cli_args::commands::init_remote::InitRemoteArgs;
use pamm_lib::application::pack::read::get_repo_info::GetRepoInfo;
use pamm_lib::application::repo::client::ClientRepoHandle;

pub async fn init_remote_command(args: InitRemoteArgs) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    let handle = ClientRepoHandle::init_from_remote(&current_dir, &args.remote).await?;

    println!("{}", handle.get_config().to_pretty_string());

    Ok(())
}
