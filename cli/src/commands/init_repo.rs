use crate::commands::input::from_cli_input::FromCliInput;
use pamm_lib::application::repo::server::ServerRepoHandle;
use pamm_lib::domain::repo::repo_config::RepoConfig;

// TODO: Allow non interactive mode with args
pub async fn init_repo_command() -> anyhow::Result<()> {
    let repo_config = RepoConfig::from_cli_input()?;

    ServerRepoHandle::create(&std::env::current_dir()?, repo_config).await?;

    println!("Created repo");

    Ok(())
}
