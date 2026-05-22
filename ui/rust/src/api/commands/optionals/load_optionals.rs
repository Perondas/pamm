use flutter_rust_bridge::frb;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::optionals::load_optionals::LoadOptionals;
pub use pamm_lib::handle::optionals::optional_addon::OptionalAddon;

pub fn load_optionals(repot_path: String, pack_name: String) -> anyhow::Result<Vec<OptionalAddon>> {
    let repot_path = std::path::Path::new(&repot_path);

    let handle = ClientRepoHandle::open(repot_path)?;

    handle.load_optionals(&pack_name)
}

#[frb(mirror(OptionalAddon))]
pub struct _OptionalAddon {
    pub name: String,
    pub enabled: bool,
}
