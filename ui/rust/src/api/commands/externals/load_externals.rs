use flutter_rust_bridge::frb;
pub use pamm_lib::handle::client::externals::external_addon::ExternalAddon;
use pamm_lib::handle::client::externals::load_externals::LoadExternals;
use pamm_lib::handle::client::client_repo_handle::ClientRepoHandle;

pub fn load_externals(repot_path: String, pack_name: String) -> anyhow::Result<Vec<ExternalAddon>> {
    let repot_path = std::path::Path::new(&repot_path);

    let handle = ClientRepoHandle::open(repot_path)?;

    handle.load_externals(&pack_name)
}

#[frb(mirror(ExternalAddon))]
pub struct _ExternalAddon {
    pub path: String,
    pub name: Option<String>,
    pub enabled: bool,
}
