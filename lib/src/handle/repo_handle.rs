#[cfg(feature = "client")]
use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::handle::reading::get_repo_info::GetRepoInfo;
#[cfg(feature = "server")]
use crate::handle::server_repo_handle::ServerRepoHandle;
use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::files::name_consts::ADDONS_DIR_NAME;
use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::fs::migration::run_migrations;
use crate::io::fs::util::path_checks::{is_dir, is_file};
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;
use crate::models::server_config::ServerConfig;
use anyhow::{Context, ensure};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum RepoHandle {
    #[cfg(feature = "client")]
    Client(ClientRepoHandle),
    #[cfg(feature = "server")]
    Server(ServerRepoHandle),
}

#[derive(Debug)]
pub struct RepoBase {
    pub repo_path: PathBuf,
    pub(crate) repo_config: RepoConfig,
}

pub(crate) trait ConcreteRepoHandle:
    Deref<Target = RepoBase> + DerefMut + Send + Sync
{
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

#[cfg(feature = "client")]
impl From<ClientRepoHandle> for RepoHandle {
    fn from(handle: ClientRepoHandle) -> Self {
        Self::Client(handle)
    }
}

#[cfg(feature = "server")]
impl From<ServerRepoHandle> for RepoHandle {
    fn from(handle: ServerRepoHandle) -> Self {
        Self::Server(handle)
    }
}

impl RepoHandle {
    #[cfg(all(feature = "client", feature = "server"))]
    pub async fn open(repo_path: &Path) -> anyhow::Result<Self> {
        match check_repo(repo_path).await? {
            RepoKind::Server => Ok(ServerRepoHandle::open(repo_path).await?.into()),
            RepoKind::Client => Ok(ClientRepoHandle::open(repo_path).await?.into()),
        }
    }

    #[cfg(all(feature = "client", not(feature = "server")))]
    pub async fn open(repo_path: &Path) -> anyhow::Result<Self> {
        match check_repo(repo_path).await? {
            RepoKind::Client => Ok(ClientRepoHandle::open(repo_path).await?.into()),
            RepoKind::Server => {
                anyhow::bail!("Repository is a server repo, but the `server` feature is disabled")
            }
        }
    }

    #[cfg(all(feature = "server", not(feature = "client")))]
    pub async fn open(repo_path: &Path) -> anyhow::Result<Self> {
        match check_repo(repo_path).await? {
            RepoKind::Server => Ok(ServerRepoHandle::open(repo_path).await?.into()),
            RepoKind::Client => {
                anyhow::bail!("Repository is a client repo, but the `client` feature is disabled")
            }
        }
    }
}

impl Deref for RepoHandle {
    type Target = RepoBase;

    fn deref(&self) -> &Self::Target {
        match self {
            #[cfg(feature = "client")]
            Self::Client(handle) => handle,
            #[cfg(feature = "server")]
            Self::Server(handle) => handle,
        }
    }
}

impl DerefMut for RepoHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            #[cfg(feature = "client")]
            Self::Client(handle) => handle,
            #[cfg(feature = "server")]
            Self::Server(handle) => handle,
        }
    }
}

impl<T: ConcreteRepoHandle> GetRepoInfo for T {
    fn get_repo_path(&self) -> &Path {
        &self.repo_path
    }

    fn get_config(&self) -> &RepoConfig {
        &self.repo_config
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

impl GetRepoInfo for RepoHandle {
    fn get_repo_path(&self) -> &Path {
        match self {
            #[cfg(feature = "client")]
            Self::Client(handle) => handle.get_repo_path(),
            #[cfg(feature = "server")]
            Self::Server(handle) => handle.get_repo_path(),
        }
    }

    fn get_config(&self) -> &RepoConfig {
        match self {
            #[cfg(feature = "client")]
            Self::Client(handle) => handle.get_config(),
            #[cfg(feature = "server")]
            Self::Server(handle) => handle.get_config(),
        }
    }
}

#[derive(Clone, Copy)]
enum RepoKind {
    Client,
    Server,
}

async fn check_repo(repo_path: &Path) -> anyhow::Result<RepoKind> {
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
        !(has_server_config && has_client_settings),
        "Repository has both server and client configuration files. This is not allowed."
    );

    match (has_server_config, has_client_settings) {
        (true, false) => Ok(RepoKind::Server),
        (false, true) => Ok(RepoKind::Client),
        (false, false) => anyhow::bail!(
            "Repository has neither server nor client configuration files. This is not allowed."
        ),
        (true, true) => unreachable!("Both config files were rejected above"),
    }
}

#[cfg(all(test, feature = "client", feature = "server"))]
mod tests {
    use super::*;
    use crate::util::test::repo_builder::{default_client_repo, default_server_repo};
    use crate::util::test::test_utils::TestTempDir;

    #[tokio::test]
    async fn open_detects_client_and_server_repositories() {
        let client_tmp = TestTempDir::new("pamm_repo_handle_open_client");
        let client = default_client_repo(client_tmp.path())
            .build()
            .await
            .unwrap();
        assert!(matches!(
            RepoHandle::open(client.repo_path()).await.unwrap(),
            RepoHandle::Client(_)
        ));

        let server_tmp = TestTempDir::new("pamm_repo_handle_open_server");
        let server = default_server_repo(server_tmp.path())
            .build()
            .await
            .unwrap();
        assert!(matches!(
            RepoHandle::open(server.repo_path()).await.unwrap(),
            RepoHandle::Server(_)
        ));
    }
}
