use flutter_rust_bridge::frb;
use pamm_lib::io::net::downloadable::KnownDownloadable;
use pamm_lib::repo::repo_config::RepoConfig;
use url::Url;

pub fn get_remote_repo_info(remote: &str) -> anyhow::Result<RepoConfig> {
    let remote = Url::parse(remote)?;

    let config = RepoConfig::download_known(&remote)?;

    Ok(config)
}
