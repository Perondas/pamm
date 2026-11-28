use pamm_lib::application::pack::read::get_pack::GetPack;
use pamm_lib::application::repo::client::ClientRepoHandle;
use pamm_lib::domain::cdlc::Cdlc;
use pamm_lib::domain::pack::pack_config::PackConfig;

pub struct PackInfo {
    pub description: String,
    pub parents: Vec<String>,
    pub enabled_cdlc: Option<Cdlc>,
}

#[err_wrap]
pub async fn load_pack_info(repo_path: String, pack_name: String) -> anyhow::Result<PackInfo> {
    let repo_path = std::path::Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_path).await?;
    handle.get_pack(&pack_name).await.map(Into::into)
}

impl From<PackConfig> for PackInfo {
    fn from(value: PackConfig) -> Self {
        Self {
            description: value.description,
            parents: value.parents,
            enabled_cdlc: value.enabled_cdlc,
        }
    }
}
