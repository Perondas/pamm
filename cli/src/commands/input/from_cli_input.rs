use anyhow::Result;
use pamm_lib::repo::repo_config::RepoConfig;

pub trait FromCliInput {
    fn from_cli_input() -> Result<Self>
    where
        Self: Sized;
}

pub trait FromCliInputWithContext {
    fn from_cli_input(repo_config: &RepoConfig) -> Result<Self>
    where
        Self: Sized;
}
