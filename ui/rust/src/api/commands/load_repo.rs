use anyhow::{anyhow, Context};
use pamm_lib::io::fs::fs_readable::KnownFSReadable;
use pamm_lib::models::repo::repo_config::RepoConfig;

pub fn load_repo(repo_path: String) -> anyhow::Result<RepoConfig> {
    let repo_path = std::path::Path::new(&repo_path);

    RepoConfig::read_from_known(repo_path).context(anyhow!(
        "Repo config not found in the given path: {:#?}",
        repo_path
    ))
}
