use crate::subcommands::externals::externals_to_names;
use pamm_cli_args::subcommands::externals::remove_external::RemoveExternalArgs;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::externals::load_externals::LoadExternals;
use pamm_lib::handle::externals::save_externals::SaveExternals;

pub async fn remove_external_command(args: RemoveExternalArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&std::env::current_dir()?).await?;

    let externals = handle.load_externals(&args.name).await?;

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
