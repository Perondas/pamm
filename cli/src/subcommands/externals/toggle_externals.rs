use crate::subcommands::externals::externals_to_names;
use pamm_cli_args::subcommands::externals::toggle_externals::ToggleExternalsArgs;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::externals::load_externals::LoadExternals;
use pamm_lib::handle::externals::save_externals::SaveExternals;

pub async fn toggle_externals_command(args: ToggleExternalsArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&std::env::current_dir()?).await?;

    let mut externals = handle.load_externals(&args.name).await?;

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
