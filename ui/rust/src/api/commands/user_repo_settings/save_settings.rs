use crate::api::commands::user_repo_settings::FlutterRepoUserSettings;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;

pub fn save_settings(repo_path: String, setting: FlutterRepoUserSettings) -> anyhow::Result<()> {
    let repo_path = std::path::Path::new(&repo_path);

    let mut handle = ClientRepoHandle::open(repo_path)?;

    handle.update_settings(setting.try_into()?)
}
