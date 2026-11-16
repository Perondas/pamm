use crate::application::pack::read::get_repo_info::GetRepoInfo;
use crate::domain::repo::repo_config::RepoConfig;
use crate::domain::repo::repo_user_settings::RepoUserSettings;
use crate::domain::repo::server_config::ServerConfig;
use crate::infrastructure::fs::files::file_names::fixed_file::FixedFile;
#[cfg(feature = "client")]
use crate::infrastructure::fs::files::file_paths::keyed_path::KeyedFilePath;
use crate::infrastructure::fs::files::file_paths::self_identified_path::SelfIdentifiedFilePath;
use crate::infrastructure::fs::files::fs_readable::KnownFSReadable;
use crate::infrastructure::fs::files::fs_writable::FixedFsWritable;
use crate::infrastructure::fs::files::name_consts::ADDONS_DIR_NAME;
use crate::infrastructure::fs::files::util::path_checks::{is_dir, is_file};
use crate::infrastructure::fs::repo::migration::run_migrations;
use anyhow::{Context, anyhow, ensure};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct RepoBase {
    pub repo_path: PathBuf,
    pub(crate) repo_config: RepoConfig,
}

impl RepoBase {
    pub(crate) async fn load(repo_path: &Path) -> anyhow::Result<Self> {
        check_repo(repo_path)
            .await
            .context("Could not find repo files. Are you sure you are running the command in the right folder?")?;

        let repo_config = RepoConfig::read_from_known(repo_path).await?;
        run_migrations::run_migrations(repo_path, &repo_config).await?;
        Ok(Self::new(repo_path, repo_config))
    }

    pub(crate) fn new(repo_path: &Path, repo_config: RepoConfig) -> Self {
        Self {
            repo_path: repo_path.to_path_buf(),
            repo_config,
        }
    }

    /// Absolute-or-relative (like `repo_path`) path of a pack's addon directory.
    pub(crate) fn pack_addons_path(&self, pack_name: &str) -> PathBuf {
        self.repo_path.join(pack_name).join(ADDONS_DIR_NAME)
    }
}

impl GetRepoInfo for RepoBase {
    fn get_repo_path(&self) -> &Path {
        &self.repo_path
    }

    fn get_config(&self) -> &RepoConfig {
        &self.repo_config
    }
}

impl RepoBase {
    pub(in crate::application) async fn write<T: FixedFsWritable + SelfIdentifiedFilePath>(
        &self,
        value: &T,
    ) -> anyhow::Result<()> {
        let path = value.file_path().with_base_path(&self.repo_path);
        value.write_fixed(&path).await
    }

    #[cfg(feature = "client")]
    pub(in crate::application) async fn write_keyed<T: FixedFsWritable + KeyedFilePath>(
        &self,
        value: &T,
        key: &str,
    ) -> anyhow::Result<()> {
        let path = T::file_path(key).with_base_path(&self.repo_path);
        value.write_fixed(&path).await
    }
}

impl RepoBase {
    pub(in crate::application) async fn read_keyed<T: KnownFSReadable + KeyedFilePath>(
        &self,
        key: &str,
    ) -> anyhow::Result<T> {
        let path = T::file_path(key).with_base_path(&self.repo_path);
        T::read_from_known(path)
            .await
            .context(anyhow!("reading {:?}", T::file_name()))
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

    let server_config_path = repo_path.join(ServerConfig::file_name());
    let client_settings_path = repo_path.join(RepoUserSettings::file_name());
    let has_server_config = is_file(&server_config_path).await;
    let has_client_settings = is_file(&client_settings_path).await;

    ensure!(
        (has_server_config ^ has_client_settings),
        "Repository either has both server and client configuration files or neither. This is not allowed."
    );

    Ok(())
}
