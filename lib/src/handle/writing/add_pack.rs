#[cfg(feature = "client")]
use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::handle::repo_handle::RepoHandle;
#[cfg(feature = "server")]
use crate::handle::server_repo_handle::ServerRepoHandle;
use crate::handle::writing::reserved_root_names;
use crate::models::pack::pack_config::PackConfig;
use crate::util::name_matches::names_match;
use anyhow::ensure;

impl RepoHandle {
    /// Shared bookkeeping for adding a pack: ensure the name is new, register it in
    /// `repo.config.json`, and write the pack config. Does NOT lay out the on-disk
    /// addon directory — callers do that with the layout that suits their role
    /// (server vs client), since the two differ (client also writes user settings).
    pub(in crate::handle) async fn register_pack(
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

#[cfg(feature = "server")]
impl ServerRepoHandle {
    /// Add a pack to the server repo: register it and lay out the source addon
    /// directory. No client-only settings file is created.
    pub async fn add_pack(&mut self, pack_config: &PackConfig) -> anyhow::Result<()> {
        self.register_pack(pack_config).await?;
        pack_config.init_source_on_fs(&self.repo_path).await
    }
}

#[cfg(feature = "client")]
impl ClientRepoHandle {
    /// Add a pack to the client repo: register it and lay out the client addon
    /// directory (including the default user settings).
    pub async fn add_pack(&mut self, pack_config: &PackConfig) -> anyhow::Result<()> {
        self.register_pack(pack_config).await?;
        pack_config.init_client_on_fs(&self.repo_path).await
    }
}

#[cfg(all(test, feature = "server"))]
mod tests {
    use crate::handle::reading::get_repo_info::GetRepoInfo;
    use crate::handle::server_repo_handle::ServerRepoHandle;
    use crate::models::pack::pack_config::PackConfig;
    use crate::models::repo::repo_config::RepoConfig;
    use crate::util::test_utils::TestTempDir;
    use std::collections::HashSet;

    #[tokio::test]
    async fn server_add_pack_lays_out_per_pack_folder_without_settings() {
        let tmp = TestTempDir::new("pamm_server_add_pack_no_settings");
        let repo_config = RepoConfig::new("repo".to_string(), "desc".to_string(), HashSet::new());
        let mut server = ServerRepoHandle::create(tmp.path(), repo_config)
            .await
            .unwrap();

        let pack = PackConfig::new("core".to_string(), "c".to_string(), vec![], None, vec![]);
        server.add_pack(&pack).await.unwrap();

        let repo_path = server.get_repo_path().to_path_buf();
        assert!(server.get_config().packs.contains("core"));
        assert!(repo_path.join("core/pack.config.json").is_file());
        assert!(repo_path.join("core/addons").is_dir());

        // The crux: no client-only settings file on the server.
        assert!(
            !repo_path.join("core/pack.settings.json").exists(),
            "server add-pack must not create pack settings"
        );
    }
}
