use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::fs::fs_writable::FixedFsWritable;
use crate::io::fs::migrations::run_migrations;
use crate::io::files::name_consts::WWW_DIR_NAME;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::server_config::ServerConfig;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ServerRepoHandle {
    base: RepoHandle,
    // Only read by the deploy actions, but always populated so the server
    // config file is created/validated on create/open.
    #[cfg_attr(not(feature = "deploy"), allow(dead_code))]
    pub(crate) server_config: ServerConfig,
}

impl ServerRepoHandle {
    pub fn open(repo_path: &Path) -> anyhow::Result<Self> {
        let base = RepoHandle::open(repo_path)?;
        let server_config = ServerConfig::read_from_known(repo_path)?;
        // Only migrate once we know this really is a server repo (the server
        // config read above fails otherwise).
        run_migrations(repo_path, &base.repo_config)?;

        Ok(Self {
            base,
            server_config,
        })
    }

    pub fn create(parent_path: &Path, repo_config: RepoConfig) -> anyhow::Result<Self> {
        let base = RepoHandle::create_repo_folder(parent_path, repo_config)?;
        let server_config = ServerConfig::default();
        server_config.write_to(&base.repo_path)?;

        Ok(Self {
            base,
            server_config,
        })
    }

    /// Returns the path to the `www` build-output directory, i.e. `<repo_path>/www`.
    /// The path is only absolute if the handle was opened/created from an absolute
    /// path — `repo_path` is stored as given, without canonicalizing. Otherwise it
    /// is relative to whatever `repo_path` itself is relative to (usually the
    /// process working directory).
    pub fn get_www_path(&self) -> PathBuf {
        self.base.repo_path.join(WWW_DIR_NAME)
    }
}

impl Deref for ServerRepoHandle {
    type Target = RepoHandle;
    fn deref(&self) -> &RepoHandle {
        &self.base
    }
}

impl DerefMut for ServerRepoHandle {
    fn deref_mut(&mut self) -> &mut RepoHandle {
        &mut self.base
    }
}
