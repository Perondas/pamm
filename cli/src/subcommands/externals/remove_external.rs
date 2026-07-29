use crate::subcommands::externals::externals_to_name;
use clap::Args;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::externals::load_externals::LoadExternals;
use pamm_lib::handle::externals::save_externals::SaveExternals;

#[derive(Debug, Args)]
pub struct RemoveExternalArgs {
    /// Pack name
    #[arg()]
    pub name: String,
}

pub fn remove_external_command(args: RemoveExternalArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&std::env::current_dir()?)?;

    let mut externals = handle.load_externals(&args.name)?;

    let selection = dialoguer::Select::new()
        .with_prompt("What external to delete?")
        .items(externals_to_name(&externals))
        .interact()?;

    externals.remove(selection);

    handle.save_externals(&args.name, &externals)
}
