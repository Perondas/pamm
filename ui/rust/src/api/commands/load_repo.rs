use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::reading::get_repo_info::GetRepoInfo;
use pamm_lib::models::repo::repo_config::RepoConfig;

pub async fn load_repo(repo_path: String) -> anyhow::Result<RepoConfig> {
    let repo_path = std::path::Path::new(&repo_path);

    let handle: ClientRepoHandle = pamm_lib::handle::repo_handle::RepoHandle::open(repo_path)
        .await?
        .try_into()?;

    Ok(handle.get_config().to_owned())
}
