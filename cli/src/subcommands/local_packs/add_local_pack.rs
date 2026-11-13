use crate::commands::input::from_cli_input::FromCliInputWithContext;
use pamm_cli_args::subcommands::local_packs::add_local_pack::AddLocalPackArgs;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::models::pack::pack_config::PackConfig;

pub async fn add_local_pack_command(_args: AddLocalPackArgs) -> anyhow::Result<()> {
    let mut repo_handle: ClientRepoHandle =
        pamm_lib::handle::repo_handle::RepoHandle::open(&std::env::current_dir()?)
            .await?
            .try_into()?;

    let pack_config = PackConfig::from_cli_input(&repo_handle.get_all_packs())?;

    repo_handle.add_local_pack(&pack_config).await?;

    println!("Pack added");

    Ok(())
}
