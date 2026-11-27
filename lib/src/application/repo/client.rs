mod downloading;
pub mod params;

use crate::application::pack::read::get_pack::GetPack;
use crate::application::pack::write::save_pack_settings::SavePackSettings;
use crate::application::repo::handle::ConcreteRepoHandle;
use crate::application::repo::repo_base::RepoBase;
use crate::domain::pack::pack_config::PackConfig;
use crate::domain::pack::pack_user_settings::PackUserSettings;
use crate::domain::repo::repo_user_settings::RepoUserSettings;
use crate::infrastructure::fs::files::fs_readable::KnownFSReadable;
use crate::infrastructure::fs::files::fs_writable::FixedFsWritable;
use crate::infrastructure::fs::files::util::path_checks::is_dir;
use crate::infrastructure::fs::repo::init::init_client_repo_from_remote::init_client_repo_from_remote;
use crate::support::name_matches;
use anyhow::{Context, ensure};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use url::Url;

#[derive(Debug)]
pub struct ClientRepoHandle {
    base: RepoBase,
    user_settings: RepoUserSettings,
}

impl ClientRepoHandle {
    pub async fn open(repo_path: &Path) -> anyhow::Result<Self> {
        let base = RepoBase::load(repo_path).await?;

        let user_settings = RepoUserSettings::read_from_known(repo_path)
            .await
            .context("Failed to read user settings. Is this a client repository?")?;
        Ok(Self {
            base,
            user_settings,
        })
    }

    pub async fn init_from_remote(dest_path: &Path, remote: &Url) -> anyhow::Result<Self> {
        ensure!(is_dir(dest_path).await, "Destination path is not a folder");

        let repo_path = init_client_repo_from_remote(dest_path, remote).await?;

        let handle = Self::open(&repo_path)
            .await
            .context("Failed to open repo after initializing from remote")?;

        Ok(handle)
    }

    pub fn user_settings(&self) -> &RepoUserSettings {
        &self.user_settings
    }

    pub async fn update_settings(&mut self, settings: RepoUserSettings) -> anyhow::Result<()> {
        self.write(&settings)
            .await
            .context("Failed to write user settings")?;
        self.user_settings = settings;

        Ok(())
    }

    /// Whether `pack_name` is a local pack: laid out on disk like any other pack
    /// and recorded in `user.repo.settings.json`, but not backed by anything
    /// remote and therefore absent from `repo.config.json`, which mirrors the
    /// remote.
    pub fn is_local_pack(&self, pack_name: &str) -> bool {
        self.user_settings
            .local_packs
            .iter()
            .any(|p| name_matches::names_match(p, pack_name))
    }
}

impl Deref for ClientRepoHandle {
    type Target = RepoBase;
    fn deref(&self) -> &RepoBase {
        &self.base
    }
}

impl DerefMut for ClientRepoHandle {
    fn deref_mut(&mut self) -> &mut RepoBase {
        &mut self.base
    }
}

impl ConcreteRepoHandle for ClientRepoHandle {}

impl GetPack for ClientRepoHandle {
    async fn get_pack(&self, pack_name: &str) -> anyhow::Result<PackConfig> {
        // Local packs are absent from the repo config, so they have to bypass
        // its registry check; on disk they are laid out identically.
        if self.is_local_pack(pack_name) {
            return self.read_pack_config(pack_name).await;
        }

        self.get_registered_pack(pack_name).await
    }

    async fn get_pack_with_settings(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<(PackConfig, PackUserSettings)> {
        if self.is_local_pack(pack_name) {
            return Ok((
                self.read_pack_config(pack_name).await?,
                self.read_pack_settings(pack_name).await?,
            ));
        }

        self.get_registered_pack_with_settings(pack_name).await
    }

    fn get_all_pack_names(&self) -> anyhow::Result<Vec<&str>> {
        Ok(self
            .get_registered_pack_names()
            .into_iter()
            .chain(self.user_settings.local_packs.iter().map(|s| s.as_str()))
            .collect())
    }
}

impl SavePackSettings for ClientRepoHandle {
    fn save_pack_settings<'a, 'b>(
        &'a self,
        pack_name: &'a str,
        settings: &'a PackUserSettings,
    ) -> impl Future<Output = anyhow::Result<()>> + Send + 'b + use<'b> {
        let path = self.repo_path.join(pack_name);
        let settings = settings.clone();

        async move { settings.write_fixed(path).await }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repo::repo_config::RepoConfig;
    use crate::domain::repo::repo_version::RepoVersion;
    use crate::support::test::test_utils::TestTempDir;
    use std::collections::HashSet;
    use std::fs;

    // Opening a legacy flat (v1) client repo migrates it to the per-pack
    // layout, including hoisting indexes/ out of the addon dir.
    #[tokio::test]
    async fn open_migrates_v1_client_repo() {
        let tmp = TestTempDir::new("pamm_client_open_migrates_v1");
        let repo_path = tmp.path().join("repo");
        fs::create_dir_all(&repo_path).unwrap();

        let mut packs = HashSet::new();
        packs.insert("core".to_string());
        let repo_config = RepoConfig::new("repo".to_string(), "desc".to_string(), packs);
        crate::infrastructure::fs::files::fs_writable::FixedFsWritable::write_fixed(
            &repo_config,
            &repo_path,
        )
        .await
        .unwrap();
        let settings = RepoUserSettings::new(Url::parse("http://localhost/").unwrap());
        crate::infrastructure::fs::files::fs_writable::FixedFsWritable::write_fixed(
            &settings, &repo_path,
        )
        .await
        .unwrap();

        // Flat v1 client layout.
        fs::write(repo_path.join("core.pack.config.json"), b"{}").unwrap();
        fs::write(repo_path.join("core.pack.settings.json"), b"{}").unwrap();
        fs::create_dir_all(repo_path.join("core_pack_addons/@addon1")).unwrap();
        fs::create_dir_all(repo_path.join("core_pack_addons/indexes")).unwrap();
        fs::write(
            repo_path.join("core_pack_addons/indexes/checksum_index.pamm"),
            b"idx",
        )
        .unwrap();

        let _ = ClientRepoHandle::open(&repo_path).await.unwrap();

        assert!(repo_path.join("core/pack.config.json").is_file());
        assert!(repo_path.join("core/pack.settings.json").is_file());
        assert!(repo_path.join("core/addons/@addon1").is_dir());
        assert!(repo_path.join("core/indexes/checksum_index.pamm").is_file());
        assert!(!repo_path.join("core_pack_addons").exists());
        assert_eq!(
            RepoVersion::read_or_v1(&repo_path).await.unwrap(),
            RepoVersion::current()
        );
    }
}

#[cfg(test)]
mod local_pack_tests {
    use super::*;
    use crate::application::pack::read::get_combined_canonical_addon_paths::GetAddonPaths;
    use crate::domain::addon::AddonSettings;
    use crate::infrastructure::progress::progress_reporter::ProgressReporter;
    use crate::support::test::repo_builder::TestClientRepoBuilder;
    use crate::support::test::test_utils::TestTempDir;
    use std::fs;
    use std::path::PathBuf;

    #[derive(Clone, Default)]
    struct NoopProgress;
    impl ProgressReporter for NoopProgress {
        fn start_for_download(&self, _total_work: u64) {}
        fn start_without_len(&self) {}
        fn report_progress(&self, _progress: u64) {}
        fn report_message(&self, _message: &str) {}
        fn finish(&self) {}
    }

    /// A client repo holding remote pack `core` with `@core_addon`, plus a local
    /// pack `my_mission` parented on it with `@mission_addon`. Both addon folders
    /// exist on disk, as they would after a sync.
    async fn repo_with_local_pack(key: &str) -> (TestTempDir, PathBuf, ClientRepoHandle) {
        let tmp = TestTempDir::new(key);

        let mut core = PackConfig::new("core".to_string(), "d".to_string(), vec![], None, None);
        core.addons.insert(
            "@core_addon".to_string(),
            AddonSettings {
                is_optional: false,
                ..Default::default()
            },
        );
        let fixture = TestClientRepoBuilder::new(tmp.path())
            .repo_name("repo")
            .description("desc")
            .pack(core)
            .pack_file("core", "addons/@core_addon/.keep", b"".as_slice())
            .build()
            .await
            .unwrap();
        let repo_path = fixture.repo_path().to_path_buf();

        let mut handle = ClientRepoHandle::open(&repo_path).await.unwrap();

        let mut mission = PackConfig::new(
            "my_mission".to_string(),
            "d".to_string(),
            vec![],
            Some("core".to_string()),
            None,
        );
        mission.addons.insert(
            "@mission_addon".to_string(),
            AddonSettings {
                is_optional: false,
                ..Default::default()
            },
        );
        handle.add_local_pack(&mission).await.unwrap();
        fs::create_dir_all(repo_path.join("my_mission/addons/@mission_addon")).unwrap();

        (tmp, repo_path, handle)
    }

    // The whole point of a local pack: it can be launched, inheriting its remote
    // parent's addons exactly as a normal child pack would.
    #[tokio::test]
    async fn a_local_pack_resolves_its_own_and_its_remote_parents_addons() {
        let (_tmp, repo_path, handle) =
            repo_with_local_pack("pamm_local_pack_canonical_addon_paths").await;

        let paths = handle
            .get_combined_canonical_addon_paths("my_mission", false)
            .await
            .unwrap();

        let clean = |rel: &str| {
            fs::canonicalize(repo_path.join(rel))
                .unwrap()
                .to_string_lossy()
                .to_string()
        };
        let mut paths = paths.into_iter().map(|p| clean(&p)).collect::<Vec<_>>();
        paths.sort();
        let mut want = vec![
            clean("my_mission/addons/@mission_addon"),
            clean("core/addons/@core_addon"),
        ];
        want.sort();
        assert_eq!(paths, want);
    }

    // An addon the pack expects but that is not on disk means the pack is out
    // of sync, which launching must not silently paper over.
    #[tokio::test]
    async fn resolving_addon_paths_fails_when_an_addon_is_missing() {
        let (_tmp, repo_path, handle) = repo_with_local_pack("pamm_local_pack_missing_addon").await;

        fs::remove_dir_all(repo_path.join("core/addons/@core_addon")).unwrap();

        let err = handle
            .get_combined_canonical_addon_paths("my_mission", false)
            .await
            .unwrap_err();

        assert!(
            format!("{err:#}").contains("'@core_addon' of pack 'core'"),
            "unexpected error: {err:#}"
        );
    }

    // Local packs exist on this machine only, so every remote-facing operation
    // has to refuse them rather than 404 against a URL that cannot exist. The
    // guards run before any network access, so these need no server.
    #[tokio::test]
    async fn remote_operations_refuse_a_local_pack() {
        let (_tmp, _repo_path, handle) =
            repo_with_local_pack("pamm_local_pack_refuses_remote").await;

        let errors = [
            format!(
                "{:#}",
                handle
                    .quick_check_pack_up_to_date("my_mission")
                    .await
                    .unwrap_err()
            ),
            format!(
                "{:#}",
                handle
                    .get_pack_diff("my_mission", NoopProgress, false)
                    .await
                    .unwrap_err()
            ),
            format!(
                "{:#}",
                handle
                    .get_pack_and_parents_diffs("my_mission", NoopProgress, false)
                    .await
                    .unwrap_err()
            ),
        ];

        for err in errors {
            assert!(
                err.contains("is a local pack and cannot be"),
                "unexpected error: {}",
                err
            );
        }
    }
}
