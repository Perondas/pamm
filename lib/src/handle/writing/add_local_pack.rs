use crate::handle::client_repo_handle::{ClientRepoHandle, names_match};
use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::files::name_consts::{
    CACHE_DB_DIR_NAME, INDEX_DIR_NAME, MEDIA_DIR_NAME, WWW_DIR_NAME,
};
use crate::models::pack::pack_config::PackConfig;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;
use crate::models::repo::repo_version::RepoVersion;
use crate::models::server_config::ServerConfig;
use anyhow::{Context, ensure};
use std::path::{Component, Path};

/// Repo-root entries a pack folder must not shadow. The fixed file names are
/// included because a pack folder sits at the repo root alongside them.
fn reserved_root_names() -> [&'static str; 8] {
    [
        WWW_DIR_NAME,
        MEDIA_DIR_NAME,
        CACHE_DB_DIR_NAME,
        INDEX_DIR_NAME,
        RepoConfig::file_name(),
        RepoUserSettings::file_name(),
        ServerConfig::file_name(),
        RepoVersion::file_name(),
    ]
}

impl ClientRepoHandle {
    /// Create a pack that exists only in this client repo. It is laid out on disk
    /// exactly like a synced pack, but is not backed by anything remote: it is
    /// recorded in `user.repo.settings.json` rather than `repo.config.json`, so
    /// config sync never sees it and can never delete it. Its parent, if any, may
    /// be a normal or another local pack.
    pub fn add_local_pack(&mut self, pack_config: &PackConfig) -> anyhow::Result<()> {
        self.validate_new_local_pack(pack_config)?;

        // Lay out the folder before recording the name. Neither order is atomic;
        // this way a failed settings write leaves an inert unclaimed folder that
        // the availability check will flag, rather than a name in `local_packs`
        // pointing at nothing.
        pack_config
            .init_client_on_fs(&self.repo_path)
            .context("Failed to lay out the local pack on disk")?;

        let mut settings = self.user_settings().clone();
        settings.local_packs.push(pack_config.name.clone());

        self.update_settings(settings)
    }

    fn validate_new_local_pack(&self, pack_config: &PackConfig) -> anyhow::Result<()> {
        let name = &pack_config.name;

        ensure!(!name.is_empty(), "Pack name must not be empty");
        ensure!(
            name.trim() == name,
            "Pack name '{}' must not start or end with whitespace",
            name
        );

        // The name is used directly as a folder name, so it has to be exactly one
        // ordinary path component: this rejects separators, `.`, `..`, absolute
        // paths and Windows prefixes.
        let mut components = Path::new(name).components();
        ensure!(
            matches!(components.next(), Some(Component::Normal(c)) if c == name.as_str())
                && components.next().is_none(),
            "Pack name '{}' is not a valid folder name",
            name
        );

        if let Some(reserved) = reserved_root_names()
            .into_iter()
            .find(|reserved| names_match(reserved, name))
        {
            anyhow::bail!(
                "Pack must not be named '{}': its folder would collide with '{}' in the repo root",
                name,
                reserved
            );
        }

        if let Some(existing) = self
            .get_config()
            .packs
            .iter()
            .find(|existing| names_match(existing, name))
        {
            anyhow::bail!(
                "Pack '{}' already exists in repo (as remote pack '{}')",
                name,
                existing
            );
        }

        if let Some(existing) = self
            .user_settings()
            .local_packs
            .iter()
            .find(|existing| names_match(existing, name))
        {
            anyhow::bail!(
                "Pack '{}' already exists in repo (as local pack '{}')",
                name,
                existing
            );
        }

        ensure!(
            !self.repo_path.join(name).exists(),
            "A file or folder named '{}' already exists in the repo",
            name
        );

        if let Some(parent) = &pack_config.parent {
            // Resolves both normal and local packs, so a local pack may parent
            // another. No cycle is possible: the new name is not yet in any chain.
            self.get_pack(parent).context(format!(
                "Parent pack '{}' of new local pack '{}' does not exist",
                parent, name
            ))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::fs::fs_readable::KnownFSReadable;
    use crate::io::fs::fs_writable::FixedFsWritable;
    use crate::util::test_utils::TestTempDir;
    use std::collections::HashSet;
    use std::fs;
    use std::path::PathBuf;
    use url::Url;

    /// A client repo at layout v2 holding the remote packs named in `remote_packs`,
    /// each laid out on disk. Written through the real writers so `open` sees a
    /// well-formed repo and its migration pass is a no-op.
    fn client_repo(key: &str, remote_packs: &[&str]) -> (TestTempDir, PathBuf) {
        let tmp = TestTempDir::new(key);
        let repo_path = tmp.path().join("repo");
        fs::create_dir_all(&repo_path).unwrap();

        let packs: HashSet<String> = remote_packs.iter().map(|p| p.to_string()).collect();
        let repo_config = RepoConfig::new("repo".to_string(), "desc".to_string(), packs);
        repo_config.write_fixed(&repo_path).unwrap();

        RepoUserSettings::new(Url::parse("http://localhost/").unwrap())
            .write_fixed(&repo_path)
            .unwrap();
        RepoVersion::current().write_fixed(&repo_path).unwrap();

        for pack in remote_packs {
            PackConfig::new(pack.to_string(), "d".to_string(), vec![], None)
                .init_client_on_fs(&repo_path)
                .unwrap();
        }

        (tmp, repo_path)
    }

    fn pack(name: &str, parent: Option<&str>) -> PackConfig {
        PackConfig::new(
            name.to_string(),
            "a local pack".to_string(),
            vec![],
            parent.map(|p| p.to_string()),
        )
    }

    #[test]
    fn lays_out_the_client_layout_and_records_the_name_outside_the_repo_config() {
        let (_tmp, repo_path) = client_repo("pamm_add_local_pack_layout", &["core"]);
        let mut handle = ClientRepoHandle::open(&repo_path).unwrap();

        handle
            .add_local_pack(&pack("my_mission", Some("core")))
            .unwrap();

        // Identical on-disk shape to a synced pack.
        let pack_dir = repo_path.join("my_mission");
        assert!(pack_dir.join("pack.config.json").is_file());
        assert!(pack_dir.join("pack.settings.json").is_file());
        assert!(pack_dir.join("addons").is_dir());
        assert!(pack_dir.join("indexes").is_dir());

        // Recorded in the user settings, and persisted there.
        assert_eq!(handle.user_settings().local_packs, vec!["my_mission"]);
        let on_disk = RepoUserSettings::read_from_known(&repo_path).unwrap();
        assert_eq!(on_disk.local_packs, vec!["my_mission"]);

        // The crux: absent from the repo config, which mirrors the remote, so
        // config sync can never see it and can never delete it.
        assert!(!handle.get_config().packs.contains("my_mission"));
        let on_disk = RepoConfig::read_from_known(&repo_path).unwrap();
        assert_eq!(on_disk.packs, HashSet::from(["core".to_string()]));
    }

    #[test]
    fn resolves_like_any_other_pack_once_created() {
        let (_tmp, repo_path) = client_repo("pamm_add_local_pack_resolves", &["core"]);
        let mut handle = ClientRepoHandle::open(&repo_path).unwrap();

        handle
            .add_local_pack(&pack("my_mission", Some("core")))
            .unwrap();

        assert!(handle.is_local_pack("my_mission"));
        assert!(!handle.is_local_pack("core"));

        let config = handle.get_pack("my_mission").unwrap();
        assert_eq!(config.name, "my_mission");
        assert_eq!(config.parent, Some("core".to_string()));

        let (config, settings) = handle.get_pack_with_settings("my_mission").unwrap();
        assert_eq!(config.name, "my_mission");
        assert!(settings.enabled_optionals.is_empty());
    }

    #[test]
    fn survives_a_reopen() {
        let (_tmp, repo_path) = client_repo("pamm_add_local_pack_reopen", &[]);
        let mut handle = ClientRepoHandle::open(&repo_path).unwrap();
        handle.add_local_pack(&pack("my_mission", None)).unwrap();
        drop(handle);

        let handle = ClientRepoHandle::open(&repo_path).unwrap();
        assert!(handle.is_local_pack("my_mission"));
        assert_eq!(handle.get_pack("my_mission").unwrap().name, "my_mission");
    }

    #[test]
    fn a_local_pack_may_parent_another_local_pack() {
        let (_tmp, repo_path) = client_repo("pamm_add_local_pack_local_parent", &[]);
        let mut handle = ClientRepoHandle::open(&repo_path).unwrap();

        handle.add_local_pack(&pack("base", None)).unwrap();
        handle
            .add_local_pack(&pack("derived", Some("base")))
            .unwrap();

        assert_eq!(
            handle.get_pack("derived").unwrap().parent,
            Some("base".to_string())
        );
    }

    fn rejects(key: &str, remote_packs: &[&str], config: PackConfig, expected: &str) {
        let (_tmp, repo_path) = client_repo(key, remote_packs);
        let mut handle = ClientRepoHandle::open(&repo_path).unwrap();

        let err = handle.add_local_pack(&config).unwrap_err();
        let err = format!("{:#}", err);
        assert!(
            err.contains(expected),
            "expected error containing {:?}, got: {}",
            expected,
            err
        );

        // A rejected pack leaves nothing behind.
        assert!(handle.user_settings().local_packs.is_empty());
    }

    #[test]
    fn rejects_the_name_of_an_existing_remote_pack() {
        rejects(
            "pamm_add_local_pack_dup_remote",
            &["core"],
            pack("core", None),
            "already exists in repo",
        );
    }

    #[test]
    fn rejects_a_remote_name_differing_only_in_case() {
        rejects(
            "pamm_add_local_pack_dup_remote_case",
            &["core"],
            pack("Core", None),
            "as remote pack 'core'",
        );
    }

    #[test]
    fn rejects_the_name_of_an_existing_local_pack() {
        let (_tmp, repo_path) = client_repo("pamm_add_local_pack_dup_local", &[]);
        let mut handle = ClientRepoHandle::open(&repo_path).unwrap();
        handle.add_local_pack(&pack("my_mission", None)).unwrap();

        // Differing only in case, which would share one folder on NTFS/APFS.
        let err = format!(
            "{:#}",
            handle
                .add_local_pack(&pack("My_Mission", None))
                .unwrap_err()
        );
        assert!(
            err.contains("as local pack 'my_mission'"),
            "unexpected error: {}",
            err
        );
        assert_eq!(handle.user_settings().local_packs, vec!["my_mission"]);
    }

    #[test]
    fn rejects_reserved_root_names() {
        for (i, name) in ["www", "media", "indexes", ".cache"].iter().enumerate() {
            rejects(
                &format!("pamm_add_local_pack_reserved_{}", i),
                &[],
                pack(name, None),
                "would collide with",
            );
        }
        for (i, name) in [
            "repo.config.json",
            "user.repo.settings.json",
            "server.config.json",
            "version.pamm",
        ]
        .iter()
        .enumerate()
        {
            rejects(
                &format!("pamm_add_local_pack_reserved_file_{}", i),
                &[],
                pack(name, None),
                "would collide with",
            );
        }
    }

    #[test]
    fn rejects_path_like_names() {
        for (i, name) in ["..", ".", "a/b", "../escape", "/abs"].iter().enumerate() {
            rejects(
                &format!("pamm_add_local_pack_pathlike_{}", i),
                &[],
                pack(name, None),
                "is not a valid folder name",
            );
        }
    }

    #[test]
    fn rejects_an_empty_or_padded_name() {
        rejects(
            "pamm_add_local_pack_empty",
            &[],
            pack("", None),
            "must not be empty",
        );
        rejects(
            "pamm_add_local_pack_padded",
            &[],
            pack(" spaced ", None),
            "must not start or end with whitespace",
        );
    }

    #[test]
    fn rejects_a_name_whose_folder_already_exists() {
        let (_tmp, repo_path) = client_repo("pamm_add_local_pack_folder_taken", &[]);
        fs::create_dir_all(repo_path.join("squatter")).unwrap();

        let mut handle = ClientRepoHandle::open(&repo_path).unwrap();
        let err = format!(
            "{:#}",
            handle.add_local_pack(&pack("squatter", None)).unwrap_err()
        );
        assert!(
            err.contains("already exists in the repo"),
            "unexpected error: {}",
            err
        );
    }

    #[test]
    fn rejects_a_parent_that_does_not_exist() {
        rejects(
            "pamm_add_local_pack_missing_parent",
            &["core"],
            pack("my_mission", Some("nope")),
            "Parent pack 'nope' of new local pack 'my_mission' does not exist",
        );
    }
}
