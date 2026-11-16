use crate::subcommands::externals::externals_to_names;
use pamm_cli_args::subcommands::externals::toggle_externals::ToggleExternalsArgs;
use pamm_lib::application::addons::externals::GetAllStoredExternals;
use pamm_lib::application::addons::externals::SaveExternals;
use pamm_lib::application::repo::client::ClientRepoHandle;
use std::env::current_dir;

pub async fn toggle_externals_command(args: ToggleExternalsArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&current_dir()?).await?;

    let mut externals = handle.get_all_stored_externals(&args.name).await?;

    if externals.is_empty() {
        println!("No externals to toggle.");
        return Ok(());
    }

    let selection = dialoguer::MultiSelect::new()
        .with_prompt("What externals to enable?")
        .items(externals_to_names(&externals))
        .defaults(&externals.iter().map(|e| e.enabled).collect::<Vec<_>>())
        .interact()?;

    externals.iter_mut().enumerate().for_each(|(i, e)| {
        e.enabled = selection.contains(&i);
    });

    handle.save_externals(&args.name, &externals).await
}
