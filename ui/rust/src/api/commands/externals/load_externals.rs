use pamm_lib::application::addons::externals::GetAllStoredExternals;
use pamm_lib::application::repo::client::ClientRepoHandle;

#[err_wrap]
pub async fn load_externals(
    repot_path: String,
    pack_name: String,
) -> anyhow::Result<Vec<FlutterExternalAddon>> {
    let repot_path = std::path::Path::new(&repot_path);

    let handle = ClientRepoHandle::open(repot_path).await?;

    let externals = handle.get_all_stored_externals(&pack_name).await?;

    Ok(externals.into_iter().map(Into::into).collect())
}

pub struct FlutterExternalAddon {
    pub path: String,
    pub name: Option<String>,
    pub enabled: bool,
    pub found_in_workshop: bool,
}

impl From<pamm_lib::application::addons::externals::external_addon::ExternalAddon>
    for FlutterExternalAddon
{
    fn from(
        external: pamm_lib::application::addons::externals::external_addon::ExternalAddon,
    ) -> Self {
        Self {
            enabled: external.enabled,
            name: external.name,
            path: external.path.to_string_lossy().to_string(),
            found_in_workshop: external.found_in_workshop,
        }
    }
}
