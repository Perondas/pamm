use crate::application::pack::read::get_repo_info::GetRepoInfo;
#[cfg(feature = "client")]
use crate::application::repo::client::ClientRepoHandle;
use crate::application::repo::repo_base::RepoBase;
#[cfg(feature = "server")]
use crate::application::repo::server::ServerRepoHandle;
use crate::domain::repo::repo_config::RepoConfig;
#[cfg(all(feature = "client", feature = "server"))]
use {
    crate::domain::repo::repo_user_settings::RepoUserSettings,
    crate::domain::repo::server_config::ServerConfig,
    crate::infrastructure::fs::files::file_names::fixed_file::FixedFile,
    crate::infrastructure::fs::files::util::path_checks::is_file,
};

use std::ops::{Deref, DerefMut};
use std::path::Path;

#[derive(Debug)]
pub enum RepoHandle {
    #[cfg(feature = "client")]
    Client(ClientRepoHandle),
    #[cfg(feature = "server")]
    Server(ServerRepoHandle),
}

#[cfg(all(feature = "client", feature = "server"))]
#[derive(Clone, Copy)]
enum RepoKind {
    Client,
    Server,
}

pub(crate) trait ConcreteRepoHandle:
    Deref<Target = RepoBase> + DerefMut + Send + Sync
{
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
        match determine_repo_type(repo_path).await? {
            RepoKind::Server => Ok(ServerRepoHandle::open(repo_path).await?.into()),
            RepoKind::Client => Ok(ClientRepoHandle::open(repo_path).await?.into()),
        }
    }

    #[cfg(all(feature = "client", not(feature = "server")))]
    pub async fn open(repo_path: &Path) -> anyhow::Result<Self> {
        ClientRepoHandle::open(repo_path).await.map(Into::into)
    }

    #[cfg(all(feature = "server", not(feature = "client")))]
    pub async fn open(repo_path: &Path) -> anyhow::Result<Self> {
        ServerRepoHandle::open(repo_path).await.map(Into::into)
    }
}

#[cfg(all(feature = "client", feature = "server"))]
async fn determine_repo_type(repo_path: &Path) -> anyhow::Result<RepoKind> {
    let server_config_path = repo_path.join(ServerConfig::file_name());
    let client_settings_path = repo_path.join(RepoUserSettings::file_name());
    let has_server_config = is_file(&server_config_path).await;
    let has_client_settings = is_file(&client_settings_path).await;

    match (has_server_config, has_client_settings) {
        (true, true) => anyhow::bail!(
            "Repository has both server and client configuration files. This is not allowed."
        ),
        (true, false) => Ok(RepoKind::Server),
        (false, true) => Ok(RepoKind::Client),
        (false, false) => anyhow::bail!(
            "Repository has neither server nor client configuration files. This is not allowed."
        ),
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

#[cfg(all(test, feature = "client", feature = "server"))]
mod tests {
    use super::*;
    use crate::support::test::repo_builder::{default_client_repo, default_server_repo};
    use crate::support::test::test_utils::TestTempDir;

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
