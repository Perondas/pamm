use pamm_cli_args::subcommands::local_packs::remove_local_pack::RemoveLocalPackArgs;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;

pub async fn remove_local_pack_command(args: RemoveLocalPackArgs) -> anyhow::Result<()> {
    let mut repo_handle = ClientRepoHandle::open(&std::env::current_dir()?).await?;

    repo_handle.remove_local_pack(&args.name).await
}
