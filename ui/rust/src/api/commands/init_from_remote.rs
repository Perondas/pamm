use flutter_rust_bridge::for_generated::anyhow;
use flutter_rust_bridge::frb;
use pamm_lib::handle::reading::get_repo_info::GetRepoInfo;
use pamm_lib::handle::repo_handle::RepoHandle;
pub use pamm_lib::models::repo::repo_config::RepoConfig;
use std::collections::HashSet;
use std::path::Path;
use url::Url;

pub fn init_from_remote(remote: &str, target_dir: &str) -> anyhow::Result<RepoConfig> {
    let current_dir = Path::new(target_dir);
    let remote = Url::parse(remote)?;

    let handle = RepoHandle::init_from_remote(current_dir, &remote)?;

    Ok(handle.get_config().to_owned())
}

#[frb(mirror(RepoConfig))]
pub struct _RepoConfig {
    pub name: String,
    pub description: String,
    pub packs: HashSet<String>,
}
