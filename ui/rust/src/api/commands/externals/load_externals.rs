use flutter_rust_bridge::frb;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
pub use pamm_lib::handle::externals::external_addon::ExternalAddon;
use pamm_lib::handle::externals::load_externals::LoadExternals;

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
