use crate::api::commands::user_repo_settings::FlutterRepoUserSettings;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
pub use pamm_lib::models::repo::repo_user_settings::RepoUserSettings;

pub fn load_settings(repot_path: String) -> anyhow::Result<FlutterRepoUserSettings> {
    let repot_path = std::path::Path::new(&repot_path);

    let handle = ClientRepoHandle::open(repot_path)?;

    Ok(handle.user_settings().clone().into())
}
