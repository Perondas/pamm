use crate::subcommands::externals::externals_to_names;
use pamm_cli_args::subcommands::externals::remove_external::RemoveExternalArgs;
use pamm_lib::application::addons::externals::GetAllStoredExternals;
use pamm_lib::application::addons::externals::SaveExternals;
use pamm_lib::application::repo::client::ClientRepoHandle;
use std::env::current_dir;

pub async fn remove_external_command(args: RemoveExternalArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&current_dir()?).await?;

    let externals = handle.get_all_stored_externals(&args.name).await?;

    let mut externals = externals
        .into_iter()
        .filter(|e| !e.found_in_workshop)
        .collect::<Vec<_>>();

    if externals.is_empty() {
        println!("No externals to remove.");
        return Ok(());
    }

    let selection = dialoguer::Select::new()
        .with_prompt("What external to remove?")
        .items(externals_to_names(&externals))
        .interact()?;

    externals.remove(selection);

    handle.save_externals(&args.name, &externals).await
}
