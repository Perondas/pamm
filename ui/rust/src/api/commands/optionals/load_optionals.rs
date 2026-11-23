use flutter_rust_bridge::frb;
use pamm_lib::application::addons::optionals::GetOptionals;
pub use pamm_lib::application::addons::optionals::OptionalAddon;
use pamm_lib::application::repo::client::ClientRepoHandle;

#[err_wrap]
pub async fn load_optionals(
    repot_path: String,
    pack_name: String,
) -> anyhow::Result<Vec<OptionalAddon>> {
    let repot_path = std::path::Path::new(&repot_path);

    let handle = ClientRepoHandle::open(repot_path).await?;

    handle.get_optionals(&pack_name).await
}

#[frb(mirror(OptionalAddon))]
pub struct _OptionalAddon {
    pub name: String,
    pub enabled: bool,
}
