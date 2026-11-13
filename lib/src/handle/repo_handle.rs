use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::files::name_consts::ADDONS_DIR_NAME;
use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::fs::migration::run_migrations;
use crate::io::fs::util::path_checks::{is_dir, is_file};
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;
use crate::models::server_config::ServerConfig;
use anyhow::{Context, ensure};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct RepoHandle {
    pub repo_path: PathBuf,
    pub(crate) repo_config: RepoConfig,
    pub(crate) repo_type: RepoType,
}

#[derive(Debug)]
pub enum RepoType {
    Server {
        server_config: ServerConfig,
    },
    Client {
        repo_user_settings: RepoUserSettings,
    },
}

impl RepoHandle {
    pub async fn open(repo_path: &Path) -> anyhow::Result<Self> {
        check_repo(repo_path).await.context("Could not find repo files. Are you sure you are running the command in the right folder?")?;

        let repo_config = RepoConfig::read_from_known(repo_path).await?;

        run_migrations::run_migrations(repo_path, &repo_config).await?;
        let repo_type = determine_repo_type(repo_path).await?;

        Ok(Self {
            repo_path: repo_path.to_path_buf(),
            repo_config,
            repo_type,
        })
    }

    // TODO: Extract into trait
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

async fn determine_repo_type(repo_path: &Path) -> anyhow::Result<RepoType> {
    let mut server_config = None;
    let mut client_config = None;

    let server_config_path = repo_path.join(ServerConfig::file_name());
    if is_file(&server_config_path).await {
        server_config = Some(ServerConfig::read_from_known(repo_path).await?);
    }

    let repo_user_settings_path = repo_path.join(RepoUserSettings::file_name());
    if is_file(&repo_user_settings_path).await {
        client_config = Some(RepoUserSettings::read_from_known(repo_path).await?);
    }

    if client_config.is_some() && server_config.is_some() {
        anyhow::bail!(
            "Repository has both server and client configuration files. This is not allowed."
        );
    }

    if client_config.is_none() && server_config.is_none() {
        anyhow::bail!(
            "Repository has neither server nor client configuration files. This is not allowed."
        );
    }

    if let Some(server_config) = server_config {
        Ok(RepoType::Server { server_config })
    } else if let Some(repo_user_settings) = client_config {
        Ok(RepoType::Client { repo_user_settings })
    } else {
        unreachable!("Logic error in determine_repo_type: should have returned by now");
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
