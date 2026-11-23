use crate::api::commands::user_repo_settings::FlutterRepoUserSettings;
use pamm_lib::application::repo::client::ClientRepoHandle;
pub use pamm_lib::domain::repo::repo_user_settings::RepoUserSettings;

#[err_wrap]
pub async fn load_settings(repo_path: String) -> anyhow::Result<FlutterRepoUserSettings> {
    let repot_path = std::path::Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repot_path).await?;

    Ok(handle.user_settings().clone().into())
}
