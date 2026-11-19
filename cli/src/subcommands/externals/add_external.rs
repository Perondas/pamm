use pamm_cli_args::subcommands::externals::add_external::AddExternalArgs;
use pamm_lib::application::addons::externals::ExternalAddon;
use pamm_lib::application::addons::externals::GetAllStoredExternals;
use pamm_lib::application::addons::externals::SaveExternals;
use pamm_lib::application::repo::client::ClientRepoHandle;
use std::env::current_dir;

pub async fn add_external_command(args: AddExternalArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&current_dir()?).await?;

    let mut externals = handle.get_all_stored_externals(&args.name).await?;

    let new = ExternalAddon::new(args.path)?;

    externals.push(new);

    handle.save_externals(&args.name, &externals).await
}
