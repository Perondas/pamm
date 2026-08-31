use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::handle::repo_handle::RepoHandle;
use crate::handle::writing::save_pack_settings::SavePackSettings;
use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::fs::migration::run_migrations;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_user_settings::PackUserSettings;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;
use anyhow::{Context, ensure};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use url::Url;

#[derive(Debug)]
pub struct ClientRepoHandle {
    base: RepoHandle,
    user_settings: RepoUserSettings,
}

impl ClientRepoHandle {
    pub fn open(repo_path: &Path) -> anyhow::Result<Self> {
        let base = RepoHandle::open(repo_path)?;
        ensure!(
            repo_path.join(RepoUserSettings::file_name()).exists(),
            "Client repo at {:?} is missing user.repo.settings.json. \
             Did you mean to initialize from a remote via `init-remote`?",
            repo_path
        );
        let user_settings = RepoUserSettings::read_from_known(repo_path)?;
        run_migrations::run_migrations(repo_path, &base.repo_config)?;
        Ok(Self {
            base,
            user_settings,
        })
    }

    pub fn init_from_remote(dest_path: &Path, remote: &Url) -> anyhow::Result<Self> {
        ensure!(dest_path.is_dir(), "Destination path is not a folder");

        let (repo_config, user_settings) = RepoConfig::init_from_remote(dest_path, remote)?;

        let repo_path = dest_path.join(&repo_config.name);
        let base = RepoHandle::from_parts(repo_path, repo_config);

        Ok(Self {
            base,
            user_settings,
        })
    }

    pub fn user_settings(&self) -> &RepoUserSettings {
        &self.user_settings
    }

    pub fn remote(&self) -> &Url {
        self.user_settings.get_remote()
    }

    pub fn update_settings(&mut self, settings: RepoUserSettings) -> anyhow::Result<()> {
        self.user_settings = settings;
        self.write(&self.user_settings)
            .context("Failed to write user settings")?;
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
            .any(|p| names_match(p, pack_name))
    }

    /// Guard for every operation that talks to the remote. Local packs exist
    /// only on this machine, so there is nothing to compare them against.
    pub(in crate::handle) fn ensure_not_local(&self, pack_name: &str) -> anyhow::Result<()> {
        ensure!(
            !self.is_local_pack(pack_name),
            "Pack '{}' is a local pack and has no remote; local packs can never be synced",
            pack_name
        );
        Ok(())
    }
}

/// Compare two pack names for identity. Pack names are folder names, and NTFS
/// and APFS are case-insensitive, so two names differing only in case would
/// share a single directory.
pub(in crate::handle) fn names_match(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

impl Deref for ClientRepoHandle {
    type Target = RepoHandle;
    fn deref(&self) -> &RepoHandle {
        &self.base
    }
}

impl DerefMut for ClientRepoHandle {
    fn deref_mut(&mut self) -> &mut RepoHandle {
        &mut self.base
    }
}

impl GetRepoInfo for ClientRepoHandle {
    fn get_repo_path(&self) -> &Path {
        self.base.get_repo_path()
    }

    fn get_config(&self) -> &RepoConfig {
        self.base.get_config()
    }
}

impl GetPack for ClientRepoHandle {
    fn get_pack(&self, pack_name: &str) -> anyhow::Result<PackConfig> {
        // Local packs are absent from the repo config, so they have to bypass
        // its registry check; on disk they are laid out identically.
        if self.is_local_pack(pack_name) {
            return self.base.read_pack_config(pack_name);
        }

        self.base.get_pack(pack_name)
    }

    fn get_pack_with_settings(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<(PackConfig, PackUserSettings)> {
        if self.is_local_pack(pack_name) {
            return Ok((
                self.base.read_pack_config(pack_name)?,
                self.base.read_pack_settings(pack_name)?,
            ));
        }

        self.base.get_pack_with_settings(pack_name)
    }
}

impl SavePackSettings for ClientRepoHandle {
    fn save_pack_settings(
        &self,
        pack_name: &str,
        settings: &PackUserSettings,
    ) -> anyhow::Result<()> {
        self.write_keyed(settings, pack_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::repo::repo_version::RepoVersion;
    use crate::util::test_utils::TestTempDir;
    use std::collections::HashSet;
    use std::fs;

    // Opening a legacy flat (v1) client repo migrates it to the per-pack
    // layout, including hoisting indexes/ out of the addon dir.
    #[test]
    fn open_migrates_v1_client_repo() {
        let tmp = TestTempDir::new("pamm_client_open_migrates_v1");
        let repo_path = tmp.path().join("repo");
        fs::create_dir_all(&repo_path).unwrap();

        let mut packs = HashSet::new();
        packs.insert("core".to_string());
        let repo_config = RepoConfig::new("repo".to_string(), "desc".to_string(), packs);
        crate::io::fs::fs_writable::FixedFsWritable::write_fixed(&repo_config, &repo_path).unwrap();
        let settings = RepoUserSettings::new(Url::parse("http://localhost/").unwrap());
        crate::io::fs::fs_writable::FixedFsWritable::write_fixed(&settings, &repo_path).unwrap();

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

        ClientRepoHandle::open(&repo_path).unwrap();

        assert!(repo_path.join("core/pack.config.json").is_file());
        assert!(repo_path.join("core/pack.settings.json").is_file());
        assert!(repo_path.join("core/addons/@addon1").is_dir());
        assert!(repo_path.join("core/indexes/checksum_index.pamm").is_file());
        assert!(!repo_path.join("core_pack_addons").exists());
        assert_eq!(
            RepoVersion::read_or_v1(&repo_path).unwrap(),
            RepoVersion::current()
        );
    }
}

#[cfg(test)]
mod local_pack_tests {
    use super::*;
    use crate::handle::reading::get_canonical_addon_paths::GetAddonPaths;
    use crate::io::fs::fs_writable::FixedFsWritable;
    use crate::io::progress_reporting::progress_reporter::ProgressReporter;
    use crate::models::pack::addon::AddonSettings;
    use crate::models::repo::repo_version::RepoVersion;
    use crate::util::test_utils::TestTempDir;
    use std::collections::HashSet;
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
    fn repo_with_local_pack(key: &str) -> (TestTempDir, PathBuf, ClientRepoHandle) {
        let tmp = TestTempDir::new(key);
        let repo_path = tmp.path().join("repo");
        fs::create_dir_all(&repo_path).unwrap();

        let repo_config = RepoConfig::new(
            "repo".to_string(),
            "desc".to_string(),
            HashSet::from(["core".to_string()]),
        );
        repo_config.write_fixed(&repo_path).unwrap();
        RepoUserSettings::new(Url::parse("http://localhost/").unwrap())
            .write_fixed(&repo_path)
            .unwrap();
        RepoVersion::current().write_fixed(&repo_path).unwrap();

        let mut core = PackConfig::new("core".to_string(), "d".to_string(), vec![], None);
        core.addons.insert(
            "@core_addon".to_string(),
            AddonSettings { is_optional: false },
        );
        core.init_client_on_fs(&repo_path).unwrap();
        fs::create_dir_all(repo_path.join("core/addons/@core_addon")).unwrap();

        let mut handle = ClientRepoHandle::open(&repo_path).unwrap();

        let mut mission = PackConfig::new(
            "my_mission".to_string(),
            "d".to_string(),
            vec![],
            Some("core".to_string()),
        );
        mission.addons.insert(
            "@mission_addon".to_string(),
            AddonSettings { is_optional: false },
        );
        handle.add_local_pack(&mission).unwrap();
        fs::create_dir_all(repo_path.join("my_mission/addons/@mission_addon")).unwrap();

        (tmp, repo_path, handle)
    }

    // The whole point of a local pack: it can be launched, inheriting its remote
    // parent's addons exactly as a normal child pack would.
    #[test]
    fn a_local_pack_resolves_its_own_and_its_remote_parents_addons() {
        let (_tmp, repo_path, handle) =
            repo_with_local_pack("pamm_local_pack_canonical_addon_paths");

        let paths = handle.get_canonical_addon_paths("my_mission").unwrap();

        let expected = |rel: &str| {
            fs::canonicalize(repo_path.join(rel))
                .unwrap()
                .to_string_lossy()
                .to_string()
        };
        let mut paths = paths;
        paths.sort();
        let mut want = vec![
            expected("my_mission/addons/@mission_addon"),
            expected("core/addons/@core_addon"),
        ];
        want.sort();
        assert_eq!(paths, want);
    }

    // Local packs exist on this machine only, so every remote-facing operation
    // has to refuse them rather than 404 against a URL that cannot exist. The
    // guards run before any network access, so these need no server.
    #[test]
    fn remote_operations_refuse_a_local_pack() {
        let (_tmp, _repo_path, handle) = repo_with_local_pack("pamm_local_pack_refuses_remote");

        let errors = [
            format!(
                "{:#}",
                handle
                    .quick_check_pack_up_to_date("my_mission")
                    .unwrap_err()
            ),
            format!(
                "{:#}",
                handle
                    .get_pack_diff("my_mission", NoopProgress, false)
                    .unwrap_err()
            ),
            format!(
                "{:#}",
                handle
                    .get_pack_and_parents_diffs("my_mission", NoopProgress, false)
                    .unwrap_err()
            ),
        ];

        for err in errors {
            assert!(
                err.contains("is a local pack and has no remote"),
                "unexpected error: {}",
                err
            );
        }
    }
}
