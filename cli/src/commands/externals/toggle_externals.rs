use clap::Args;
use pamm_lib::handle::externals::external_addon::ExternalAddon;
use pamm_lib::handle::repo_handle::RepoHandle;
use std::path::Path;

#[derive(Debug, Args)]
pub struct ToggleExternalsArgs {
    /// Pack name
    #[arg()]
    pub name: String,
}

pub fn toggle_externals_command(args: ToggleExternalsArgs) -> anyhow::Result<()> {
    let handle = RepoHandle::open(&std::env::current_dir()?)?;

    let mut externals = handle.load_externals(&args.name)?;
    
    let selection = dialoguer::MultiSelect::new()
        .with_prompt("What externals to enable?")
        .items(externals_to_name(&externals))
        .defaults(&externals.iter().map(|e| e.enabled).collect::<Vec<_>>())
        .interact()?;

    externals.iter_mut().enumerate().for_each(|(i, e)| {
        e.enabled = selection.contains(&i);
    });

    handle.save_externals(&args.name, &externals)?;

    Ok(())
}

fn externals_to_name(externals: &[ExternalAddon]) -> Vec<String> {
    externals
        .iter()
        .map(|e| {
            e.name.clone().unwrap_or(
                Path::new(&e.path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
            )
        })
        .collect()
}
