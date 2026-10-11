use pamm_lib::io::net::downloadable::KnownDownloadable;
use pamm_lib::models::repo::repo_config::RepoConfig;
use url::Url;

pub async fn get_remote_repo_info(remote: &str) -> anyhow::Result<RepoConfig> {
    let remote = Url::parse(remote)?;

    let config = RepoConfig::download_known(&remote).await?;

    Ok(config)
}
