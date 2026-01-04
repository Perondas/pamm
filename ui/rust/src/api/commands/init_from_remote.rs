use flutter_rust_bridge::for_generated::anyhow;
use flutter_rust_bridge::frb;
pub use pamm_lib::repo::repo_config::RepoConfig;
use std::collections::HashSet;
use std::path::Path;
use url::Url;

#[frb(sync)]
pub fn init_from_remote(remote: &str, target_dir: &str) -> anyhow::Result<RepoConfig> {
    let current_dir = Path::new(target_dir);
    let remote = Url::parse(remote)?;

    let config = RepoConfig::init_from_remote(current_dir, &remote)?;

    Ok(config)
}

#[frb(mirror(RepoConfig))]
pub struct _RepoConfig {
    pub name: String,
    pub description: String,
    pub packs: HashSet<String>,
}
