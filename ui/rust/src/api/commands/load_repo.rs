use pamm_lib::handle::reading::get_repo_info::GetRepoInfo;
use pamm_lib::handle::repo_handle::RepoHandle;
use pamm_lib::models::repo::repo_config::RepoConfig;

pub fn load_repo(repo_path: String) -> anyhow::Result<RepoConfig> {
    let repo_path = std::path::Path::new(&repo_path);

    let handle = RepoHandle::open(repo_path)?;

    Ok(handle.get_config().to_owned())
}
