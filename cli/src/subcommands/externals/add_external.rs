use pamm_cli_args::subcommands::externals::add_external::AddExternalArgs;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::externals::external_addon::ExternalAddon;
use pamm_lib::handle::externals::load_externals::LoadExternals;
use pamm_lib::handle::externals::save_externals::SaveExternals;
use pamm_lib::handle::repo_handle::RepoHandle;
use std::env::current_dir;

pub async fn add_external_command(args: AddExternalArgs) -> anyhow::Result<()> {
    let handle: ClientRepoHandle = RepoHandle::open(&current_dir()?).await?.try_into()?;

    let mut externals = handle.load_externals(&args.name).await?;

    let new = ExternalAddon::new(args.path)?;

    externals.push(new);

    handle.save_externals(&args.name, &externals).await
}
