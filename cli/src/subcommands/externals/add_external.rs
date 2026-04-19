use clap::Args;
use pamm_lib::handle::externals::external_addon::ExternalAddon;
use pamm_lib::handle::externals::load_externals::LoadExternals;
use pamm_lib::handle::externals::save_externals::SaveExternals;
use pamm_lib::handle::repo_handle::RepoHandle;

#[derive(Debug, Args)]
pub struct AddExternalArgs {
    /// Pack name
    #[arg()]
    pub name: String,

    #[arg()]
    pub path: String,
}

pub fn add_external_command(args: AddExternalArgs) -> anyhow::Result<()> {
    let handle = RepoHandle::open(&std::env::current_dir()?)?;

    let mut externals = handle.load_externals(&args.name)?;

    let mut new = ExternalAddon::new(args.path);

    new.resolve_name()?;

    externals.push(new);

    handle.save_externals(&args.name, &externals)?;

    Ok(())
}
