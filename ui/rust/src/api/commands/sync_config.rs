use pamm_lib::actions::sync::interactor::DummyConfigSyncInteractor;
use pamm_lib::actions::sync::sync_pack::sync_pack_config;
use pamm_lib::models::repo::repo_config::RepoConfig;
use std::path::Path;

pub fn sync_config(repo_path: String) -> anyhow::Result<RepoConfig> {
    let repo_path = Path::new(&repo_path);

    sync_pack_config(repo_path, &DummyConfigSyncInteractor)
}
