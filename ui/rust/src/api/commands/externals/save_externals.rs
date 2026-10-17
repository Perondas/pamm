use crate::api::commands::externals::load_externals::FlutterExternalAddon;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::externals::external_addon::ExternalAddon;
use pamm_lib::handle::externals::save_externals::SaveExternals;

pub async fn save_externals(
    repo_path: String,
    pack_name: String,
    externals: Vec<FlutterExternalAddon>,
) -> anyhow::Result<()> {
    let repo_path = std::path::Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_path).await?;

    let externals = externals.into_iter().map(Into::into).collect::<Vec<_>>();

    handle.save_externals(&pack_name, &externals).await
}

impl From<FlutterExternalAddon> for ExternalAddon {
    fn from(value: FlutterExternalAddon) -> Self {
        ExternalAddon::new(value.path).with_enabled(value.enabled)
    }
}
