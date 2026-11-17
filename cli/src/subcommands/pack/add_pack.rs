use crate::commands::input::from_cli_input::FromCliInputWithContext;
use pamm_cli_args::subcommands::pack::add_pack::AddPackArgs;
use pamm_lib::application::pack::read::get_pack::GetPack;
use pamm_lib::application::repo::server::ServerRepoHandle;
use pamm_lib::domain::pack::pack_config::PackConfig;

pub async fn add_pack_command(_args: AddPackArgs) -> anyhow::Result<()> {
    let mut repo_handle = ServerRepoHandle::open(&std::env::current_dir()?).await?;

    let pack_config = PackConfig::from_cli_input(&repo_handle.get_all_pack_names()?)?;

    repo_handle.add_pack(&pack_config).await?;

    println!("Pack added");

    Ok(())
}
