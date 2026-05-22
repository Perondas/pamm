use clap::Args;
use pamm_lib::handle::client::externals::external_addon::ExternalAddon;
use pamm_lib::handle::client::externals::load_externals::LoadExternals;
use pamm_lib::handle::client::externals::save_externals::SaveExternals;
use pamm_lib::handle::client::client_repo_handle::ClientRepoHandle;

#[derive(Debug, Args)]
pub struct AddExternalArgs {
    /// Pack name
    #[arg()]
    pub name: String,

    #[arg()]
    pub path: String,
}

pub fn add_external_command(args: AddExternalArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&std::env::current_dir()?)?;

    let mut externals = handle.load_externals(&args.name)?;

    let mut new = ExternalAddon::new(args.path);

    new.resolve_name()?;

    externals.push(new);

    handle.save_externals(&args.name, &externals)
}
