use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::externals::load_externals::LoadExternals;

pub async fn load_externals(
    repot_path: String,
    pack_name: String,
) -> anyhow::Result<Vec<FlutterExternalAddon>> {
    let repot_path = std::path::Path::new(&repot_path);

    let handle = ClientRepoHandle::open(repot_path).await?;

    let externals = handle.load_externals(&pack_name).await?;

    Ok(externals.into_iter().map(Into::into).collect())
}

pub struct FlutterExternalAddon {
    pub path: String,
    pub name: Option<String>,
    pub enabled: bool,
}

impl From<pamm_lib::handle::externals::external_addon::ExternalAddon> for FlutterExternalAddon {
    fn from(external: pamm_lib::handle::externals::external_addon::ExternalAddon) -> Self {
        Self {
            enabled: external.enabled,
            name: external.name,
            path: external.path,
        }
    }
}
