use crate::subcommands::externals::externals_to_name;
use clap::Args;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::externals::load_externals::LoadExternals;
use pamm_lib::handle::externals::save_externals::SaveExternals;

/// Interactively enable or disable external addons of a pack
#[derive(Debug, Args)]
pub struct ToggleExternalsArgs {
    /// Pack name
    #[arg()]
    pub name: String,
}

pub fn toggle_externals_command(args: ToggleExternalsArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&std::env::current_dir()?)?;

    let mut externals = handle.load_externals(&args.name)?;

    let selection = dialoguer::MultiSelect::new()
        .with_prompt("What externals to enable?")
        .items(externals_to_name(&externals))
        .defaults(&externals.iter().map(|e| e.enabled).collect::<Vec<_>>())
        .interact()?;

    externals.iter_mut().enumerate().for_each(|(i, e)| {
        e.enabled = selection.contains(&i);
    });

    handle.save_externals(&args.name, &externals)
}
