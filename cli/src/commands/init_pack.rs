use crate::commands::input::from_cli_input::FromCliInput;
use pamm_lib::repo::repo_config::RepoConfig;

pub fn init_repo_command() -> anyhow::Result<()> {
    let repo_config = RepoConfig::from_cli_input()?;

    repo_config.init_blank_on_fs(&std::env::current_dir()?)?;

    Ok(())
}
