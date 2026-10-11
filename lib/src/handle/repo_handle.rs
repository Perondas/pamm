use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::files::name_consts::ADDONS_DIR_NAME;
use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::fs::util::path_checks::{is_dir, is_file};
use crate::models::repo::repo_config::RepoConfig;
use anyhow::{Context, ensure};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct RepoHandle {
    pub repo_path: PathBuf,
    pub(crate) repo_config: RepoConfig,
}

impl RepoHandle {
    pub async fn open(repo_path: &Path) -> anyhow::Result<Self> {
        check_repo(repo_path).await.context("Could not find repo files. Are you sure you are running the command in the right folder?")?;

        let repo_config = RepoConfig::read_from_known(repo_path).await?;

        Ok(Self {
            repo_path: repo_path.to_path_buf(),
            repo_config,
        })
    }

    #[cfg(feature = "server")]
    pub async fn create_repo_folder(
        parent_path: &Path,
        repo_config: RepoConfig,
    ) -> anyhow::Result<Self> {
        let repo_path = repo_config.init_blank_on_fs(parent_path).await?;

        Ok(Self {
            repo_path: repo_path.to_path_buf(),
            repo_config,
        })
    }

    #[cfg(feature = "client")]
    pub(in crate::handle) fn from_parts(repo_path: PathBuf, repo_config: RepoConfig) -> Self {
        Self {
            repo_path,
            repo_config,
        }
    }

    /// Absolute-or-relative (like `repo_path`) path of a pack's addon directory.
    pub(crate) fn pack_addons_path(&self, pack_name: &str) -> PathBuf {
        self.repo_path.join(pack_name).join(ADDONS_DIR_NAME)
    }
}

impl GetRepoInfo for RepoHandle {
    fn get_repo_path(&self) -> &Path {
        &self.repo_path
    }

    fn get_config(&self) -> &RepoConfig {
        &self.repo_config
    }
}

async fn check_repo(repo_path: &Path) -> anyhow::Result<()> {
    ensure!(
        tokio::fs::try_exists(repo_path).await?,
        "Repo path does not exist"
    );
    ensure!(is_dir(repo_path).await, "Repo path is not a directory");

    let config_path = repo_path.join(RepoConfig::file_name());
    ensure!(
        tokio::fs::try_exists(&config_path).await?,
        "Repo config file does not exist"
    );
    ensure!(
        is_file(&config_path).await,
        "Repo config path is not a file"
    );

    Ok(())
}
