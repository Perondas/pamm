use pamm_lib::handle::actions::sync::interactor::DummyConfigSyncInteractor;
use pamm_lib::handle::repo_handle::RepoHandle;
use pamm_lib::models::repo::repo_config::RepoConfig;
use std::path::Path;

pub fn sync_config(repo_path: String) -> anyhow::Result<RepoConfig> {
    let repo_path = Path::new(&repo_path);

    let mut handle = RepoHandle::open(repo_path)?;

    handle.sync_pack_config(&DummyConfigSyncInteractor)?;

    Ok(handle.get_config().to_owned())
}
