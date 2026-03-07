use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::known_file::KnownFile;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;
use anyhow::ensure;
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Debug)]
pub struct RepoHandle {
    pub repo_path: PathBuf,
    pub(crate) repo_config: RepoConfig,
    pub(crate) repo_user_settings: Option<RepoUserSettings>,
}

impl RepoHandle {
    pub fn open(repo_path: &Path) -> anyhow::Result<Self> {
        check_repo(repo_path)?;

        let repo_config = RepoConfig::read_from_known(repo_path)?;

        let settings = if repo_path
            .join(Path::new(RepoUserSettings::file_name()))
            .exists()
        {
            Some(RepoUserSettings::read_from_known(repo_path)?)
        } else {
            None
        };

        Ok(Self {
            repo_path: repo_path.to_path_buf(),
            repo_config,
            repo_user_settings: settings,
        })
    }

    pub fn create_repo(patent_path: &Path, repo_config: RepoConfig) -> anyhow::Result<Self> {
        let repo_path = repo_config.init_blank_on_fs(patent_path)?;

        Ok(Self {
            repo_path: repo_path.to_path_buf(),
            repo_config,
            repo_user_settings: None,
        })
    }

    pub fn init_from_remote(dest_path: &Path, remote: &Url) -> anyhow::Result<Self> {
        ensure!(dest_path.is_dir(), "Destination path is not a folder");

        let (repo_config, user_settings) = RepoConfig::init_from_remote(dest_path, remote)?;

        Ok(Self {
            repo_path: dest_path.to_path_buf(),
            repo_config,
            repo_user_settings: Some(user_settings),
        })
    }
}

fn check_repo(repo_path: &Path) -> anyhow::Result<()> {
    ensure!(repo_path.exists(), "Repo path does not exist");
    ensure!(repo_path.is_dir(), "Repo path is not a directory");

    let config_path = repo_path.join(RepoConfig::file_name());
    ensure!(config_path.exists(), "Repo config file does not exist");
    ensure!(config_path.is_file(), "Repo config path is not a file");

    // TODO: Probably add more checks

    Ok(())
}
