use crate::commands::input::from_cli_input::FromCliInput;
use pamm_lib::handle::repo_handle::RepoHandle;
use pamm_lib::models::repo::repo_config::RepoConfig;

// TODO: Allow non interactive mode with args
pub fn init_repo_command() -> anyhow::Result<()> {
    let repo_config = RepoConfig::from_cli_input()?;

    RepoHandle::create_repo(&std::env::current_dir()?, repo_config)?;

    println!("Created repo");

    Ok(())
}
