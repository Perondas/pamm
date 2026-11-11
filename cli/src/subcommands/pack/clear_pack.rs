use log::info;
use pamm_cli_args::subcommands::pack::clear_pack::ClearPackArgs;
use pamm_lib::handle::repo_handle::RepoHandle;
use pamm_lib::handle::writing::delete_pack_content::DeletePackContent;

pub async fn clear_pack_command(args: ClearPackArgs) -> anyhow::Result<()> {
    let mut repo_handle = RepoHandle::open(&std::env::current_dir()?).await?;

    repo_handle.delete_pack_content(&args.name).await?;

    info!("Pack content cleared for pack: {}", args.name);

    Ok(())
}
