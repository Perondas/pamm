use flutter_rust_bridge::for_generated::anyhow;
use flutter_rust_bridge::frb;
use pamm_lib::application::pack::read::get_repo_info::GetRepoInfo;
use pamm_lib::application::repo::client::ClientRepoHandle;
pub use pamm_lib::domain::repo::repo_config::RepoConfig;
pub use pamm_lib::domain::repo::repo_customization::RepoCustomization;
use std::collections::HashSet;
use std::path::Path;
use url::Url;

#[err_wrap]
pub async fn init_from_remote(remote: &str, target_dir: &str) -> anyhow::Result<RepoConfig> {
    let current_dir = Path::new(target_dir);
    let remote = Url::parse(remote)?;

    let handle = ClientRepoHandle::init_from_remote(current_dir, &remote).await?;

    Ok(handle.get_config().to_owned())
}

#[frb(mirror(RepoConfig))]
pub struct _RepoConfig {
    pub name: String,
    pub description: String,
    pub packs: HashSet<String>,
    pub customization: Option<RepoCustomization>,
}

#[frb(mirror(RepoCustomization))]
pub struct _RepoCustomization {
    pub color: Option<(u32, u32, u32, u32)>,
    pub icon: Option<String>,
    pub banner: Option<String>,
}
