use pamm_cli_args::subcommands::externals::add_external::AddExternalArgs;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::externals::external_addon::ExternalAddon;
use pamm_lib::handle::externals::load_externals::LoadExternals;
use pamm_lib::handle::externals::save_externals::SaveExternals;

pub fn add_external_command(args: AddExternalArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&std::env::current_dir()?)?;

    let mut externals = handle.load_externals(&args.name)?;

    let mut new = ExternalAddon::new(args.path);

    new.resolve_name()?;

    externals.push(new);

    handle.save_externals(&args.name, &externals)
}
