use crate::commands::input::from_cli_input::FromCliInputWithContext;
use clap::Args;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::reading::get_repo_info::GetRepoInfo;
use pamm_lib::models::pack::pack_config::PackConfig;

#[derive(Debug, Args)]
pub struct AddLocalPackArgs;

pub fn add_local_pack_command(_args: AddLocalPackArgs) -> anyhow::Result<()> {
    let mut repo_handle = ClientRepoHandle::open(&std::env::current_dir()?)?;

    let repo_config = repo_handle.get_config();

    let pack_config = PackConfig::from_cli_input(repo_config)?;

    repo_handle.add_local_pack(&pack_config)?;

    println!("Pack added");

    Ok(())
}
