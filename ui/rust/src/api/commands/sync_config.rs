use pamm_lib::application::pack::read::get_repo_info::GetRepoInfo;
use pamm_lib::application::pack::sync::config_sync_interactor::DummyConfigSyncInteractor;
use pamm_lib::application::repo::client::ClientRepoHandle;
use pamm_lib::domain::repo::repo_config::RepoConfig;
use std::path::Path;

#[err_wrap]
pub async fn sync_config(repo_path: String) -> anyhow::Result<RepoConfig> {
    let repo_path = Path::new(&repo_path);

    let mut handle = ClientRepoHandle::open(repo_path).await?;

    handle.sync_repo_config(&DummyConfigSyncInteractor).await?;

    Ok(handle.get_config().to_owned())
}
