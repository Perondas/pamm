use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::externals::external_addon::ExternalAddon;
use pamm_lib::handle::externals::save_externals::SaveExternals;

pub fn save_externals(
    repo_path: String,
    pack_name: String,
    externals: Vec<ExternalAddon>,
) -> anyhow::Result<()> {
    let repo_path = std::path::Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_path)?;

    handle.save_externals(&pack_name, &externals)
}
