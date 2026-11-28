#[cfg(feature = "client")]
use crate::application::repo::client::ClientRepoHandle;
use crate::application::repo::handle::RepoHandle;
use crate::application::repo::repo_base::RepoBase;
#[cfg(feature = "server")]
use crate::application::repo::server::ServerRepoHandle;
use crate::domain::pack::pack_config::PackConfig;
use crate::infrastructure::reserved_root_names::reserved_root_names;
use crate::support::name_matches::names_match;
use anyhow::ensure;

impl RepoBase {
    /// Register a pack without choosing its role-specific on-disk layout.
    pub(in crate::application) async fn register_pack(
        &mut self,
        pack_config: &PackConfig,
    ) -> anyhow::Result<()> {
        ensure!(
            !self
                .repo_config
                .packs
                .iter()
                .any(|p| names_match(p, &pack_config.name)),
            "Pack '{}' already exists in repo",
            pack_config.name
        );

        if let Some(reserved) = reserved_root_names()
            .into_iter()
            .find(|reserved| names_match(reserved, &pack_config.name))
        {
            anyhow::bail!(
                "Pack must not be named '{}': its folder would collide with '{}' in the repo root",
                pack_config.name,
                reserved
            );
        }

        self.repo_config.packs.insert(pack_config.name.clone());
        self.write(&self.repo_config).await
    }
}

impl RepoHandle {
    pub async fn add_pack(&mut self, pack_config: &PackConfig) -> anyhow::Result<()> {
        match self {
            #[cfg(feature = "client")]
            Self::Client(handle) => handle.add_pack(pack_config).await,
            #[cfg(feature = "server")]
            Self::Server(handle) => handle.add_pack(pack_config).await,
        }
    }
}

#[cfg(feature = "server")]
impl ServerRepoHandle {
    /// Add a pack to the server repo. No client-only settings file is created.
    pub async fn add_pack(&mut self, pack_config: &PackConfig) -> anyhow::Result<()> {
        self.register_pack(pack_config).await?;
        pack_config.init_source_on_fs(&self.repo_path).await
    }
}

#[cfg(feature = "client")]
impl ClientRepoHandle {
    /// Add a pack to the client repo, including its default user settings.
    pub async fn add_pack(&mut self, pack_config: &PackConfig) -> anyhow::Result<()> {
        self.register_pack(pack_config).await?;
        pack_config.init_client_on_fs(&self.repo_path).await
    }
}

#[cfg(all(test, feature = "server"))]
mod tests {
    use crate::application::pack::read::get_repo_info::GetRepoInfo;
    use crate::application::repo::server::ServerRepoHandle;
    use crate::domain::pack::pack_config::PackConfig;
    use crate::support::test::repo_builder::TestServerRepoBuilder;
    use crate::support::test::test_utils::TestTempDir;

    #[tokio::test]
    async fn server_add_pack_lays_out_per_pack_folder_without_settings() {
        let tmp = TestTempDir::new("pamm_server_add_pack_no_settings");
        let fixture = TestServerRepoBuilder::new(tmp.path())
            .repo_name("repo")
            .description("desc")
            .build()
            .await
            .unwrap();
        let mut server = ServerRepoHandle::open(fixture.repo_path()).await.unwrap();

        let pack = PackConfig::new("core".to_string(), "c".to_string(), vec![], vec![], None);
        server.add_pack(&pack).await.unwrap();

        let repo_path = server.get_repo_path().to_path_buf();
        assert!(server.get_config().packs.contains("core"));
        assert!(repo_path.join("core/pack.config.json").is_file());
        assert!(repo_path.join("core/addons").is_dir());
        assert!(!repo_path.join("core/pack.settings.json").exists());
    }
}
