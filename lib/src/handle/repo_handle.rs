use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::files::name_consts::ADDONS_DIR_NAME;
use crate::models::repo::repo_config::RepoConfig;
use anyhow::ensure;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct RepoHandle {
    pub repo_path: PathBuf,
    pub(crate) repo_config: RepoConfig,
}

impl RepoHandle {
    pub fn open(repo_path: &Path) -> anyhow::Result<Self> {
        check_repo(repo_path)?;

        let repo_config = RepoConfig::read_from_known(repo_path)?;

        Ok(Self {
            repo_path: repo_path.to_path_buf(),
            repo_config,
        })
    }

    pub fn create_repo_folder(parent_path: &Path, repo_config: RepoConfig) -> anyhow::Result<Self> {
        let repo_path = repo_config.init_blank_on_fs(parent_path)?;

        Ok(Self {
            repo_path: repo_path.to_path_buf(),
            repo_config,
        })
    }

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

fn check_repo(repo_path: &Path) -> anyhow::Result<()> {
    ensure!(repo_path.exists(), "Repo path does not exist");
    ensure!(repo_path.is_dir(), "Repo path is not a directory");

    let config_path = repo_path.join(RepoConfig::file_name());
    ensure!(config_path.exists(), "Repo config file does not exist");
    ensure!(config_path.is_file(), "Repo config path is not a file");

    Ok(())
}
