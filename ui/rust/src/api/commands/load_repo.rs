use pamm_lib::application::pack::read::get_repo_info::GetRepoInfo;
use pamm_lib::application::repo::client::ClientRepoHandle;
use pamm_lib::domain::repo::repo_config::RepoConfig;

#[err_wrap]
pub async fn load_repo(repo_path: String) -> anyhow::Result<RepoConfig> {
    let repo_path = std::path::Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_path).await?;

    Ok(handle.get_config().to_owned())
}
