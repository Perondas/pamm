use pamm_lib::handle::client::externals::external_addon::ExternalAddon;
use pamm_lib::handle::client::externals::save_externals::SaveExternals;
use pamm_lib::handle::client::client_repo_handle::ClientRepoHandle;

pub fn save_externals(
    repo_path: String,
    pack_name: String,
    externals: Vec<ExternalAddon>,
) -> anyhow::Result<()> {
    let repo_path = std::path::Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_path)?;

    handle.save_externals(&pack_name, &externals)
}
