use pamm_lib::domain::repo::repo_config::RepoConfig;
use pamm_lib::infrastructure::http::downloadable::KnownDownloadable;
use url::Url;

#[err_wrap]
pub async fn get_remote_repo_info(remote: &str) -> anyhow::Result<RepoConfig> {
    let remote = Url::parse(remote)?;

    let config = RepoConfig::download_known(&remote).await?;

    Ok(config)
}
