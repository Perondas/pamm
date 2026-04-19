use crate::commands::input::from_cli_input::FromCliInputWithContext;
use clap::Args;
use pamm_lib::handle::reading::get_repo_info::GetRepoInfo;
use pamm_lib::handle::repo_handle::RepoHandle;
use pamm_lib::models::pack::pack_config::PackConfig;

#[derive(Debug, Args)]
pub struct AddPackArgs;

pub fn add_pack_command(_args: AddPackArgs) -> anyhow::Result<()> {
    let mut repo_handle = RepoHandle::open(&std::env::current_dir()?)?;

    let repo_config = repo_handle.get_config();

    let pack_config = PackConfig::from_cli_input(repo_config)?;

    repo_handle.add_pack(&pack_config)?;

    println!("Pack added");

    Ok(())
}
