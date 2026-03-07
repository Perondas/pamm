use flutter_rust_bridge::frb;
pub use pamm_lib::handle::optionals::optional_addon::OptionalAddon;
use pamm_lib::handle::repo_handle::RepoHandle;

pub fn load_optionals(repot_path: String, pack_name: String) -> anyhow::Result<Vec<OptionalAddon>> {
    let repot_path = std::path::Path::new(&repot_path);

    let handle = RepoHandle::open(repot_path)?;

    handle.load_optionals(&pack_name)
}

#[frb(mirror(OptionalAddon))]
pub struct _OptionalAddon {
    pub name: String,
    pub enabled: bool,
}
