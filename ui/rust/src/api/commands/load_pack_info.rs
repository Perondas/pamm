use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::reading::get_pack::GetPack;
use pamm_lib::models::cdlc::Cdlc;
use pamm_lib::models::pack::pack_config::PackConfig;

pub struct PackInfo {
    pub description: String,
    pub parent: Option<String>,
    pub enabled_cdlc: Option<Cdlc>,
}

pub async fn load_pack_info(repo_path: String, pack_name: String) -> anyhow::Result<PackInfo> {
    let repo_path = std::path::Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_path).await?;
    handle.get_pack(&pack_name).await.map(Into::into)
}

impl From<PackConfig> for PackInfo {
    fn from(value: PackConfig) -> Self {
        Self {
            description: value.description,
            parent: value.parent,
            enabled_cdlc: value.enabled_cdlc,
        }
    }
}
