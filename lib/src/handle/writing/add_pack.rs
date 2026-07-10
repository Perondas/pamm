use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::handle::repo_handle::RepoHandle;
use crate::handle::server_repo_handle::ServerRepoHandle;
use crate::models::pack::pack_config::PackConfig;
use anyhow::ensure;

impl RepoHandle {
    /// Shared bookkeeping for adding a pack: ensure the name is new, register it in
    /// `repo.config.json`, and write the pack config. Does NOT lay out the on-disk
    /// addon directory — callers do that with the layout that suits their role
    /// (server vs client), since the two differ (client also writes user settings).
    pub(in crate::handle) fn register_pack(
        &mut self,
        pack_config: &PackConfig,
    ) -> anyhow::Result<()> {
        ensure!(
            !self.repo_config.packs.contains(&pack_config.name),
            "Pack '{}' already exists in repo",
            pack_config.name
        );

        self.repo_config.packs.insert(pack_config.name.clone());
        self.write(&self.repo_config)
    }
}

impl ServerRepoHandle {
    /// Add a pack to the server repo: register it and lay out the source addon
    /// directory. No client-only settings file is created.
    pub fn add_pack(&mut self, pack_config: &PackConfig) -> anyhow::Result<()> {
        self.register_pack(pack_config)?;
        pack_config.init_source_on_fs(&self.repo_path)
    }
}

impl ClientRepoHandle {
    /// Add a pack to the client repo: register it and lay out the client addon
    /// directory (including the default user settings).
    pub fn add_pack(&mut self, pack_config: &PackConfig) -> anyhow::Result<()> {
        self.register_pack(pack_config)?;
        pack_config.init_client_on_fs(&self.repo_path)
    }
}

#[cfg(test)]
mod tests {
    use crate::handle::reading::get_repo_info::GetRepoInfo;
    use crate::handle::server_repo_handle::ServerRepoHandle;
    use crate::models::pack::pack_config::PackConfig;
    use crate::models::repo::repo_config::RepoConfig;
    use crate::util::test_utils::TestTempDir;
    use std::collections::HashSet;

    #[test]
    fn server_add_pack_lays_out_per_pack_folder_without_settings() {
        let tmp = TestTempDir::new("pamm_server_add_pack_no_settings");
        let repo_config =
            RepoConfig::new("repo".to_string(), "desc".to_string(), HashSet::new());
        let mut server = ServerRepoHandle::create(tmp.path(), repo_config).unwrap();

        let pack = PackConfig::new("core".to_string(), "c".to_string(), vec![], vec![], None);
        server.add_pack(&pack).unwrap();

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
