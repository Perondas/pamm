use clap::Args;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;

#[derive(Debug, Args)]
pub struct RemoveLocalPackArgs {
    name: String,
}

pub fn remove_local_pack_command(args: RemoveLocalPackArgs) -> anyhow::Result<()> {
    let mut repo_handle = ClientRepoHandle::open(&std::env::current_dir()?)?;

    repo_handle.remove_local_pack(&args.name)
}
