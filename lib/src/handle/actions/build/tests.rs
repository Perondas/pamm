use crate::handle::actions::build::{BuildMode, BuildOptions};
use crate::handle::server_repo_handle::ServerRepoHandle;
use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::fs::fs_writable::NamedFSWritable;
use crate::io::known_file::KnownFile;
use crate::io::name_consts::{
    CACHE_DB_DIR_NAME, INDEX_DIR_NAME, WWW_DIR_NAME, get_pack_addon_directory_name,
};
use crate::io::named_file::NamedFile;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::models::index::checksum_index::ChecksumIndex;
use crate::models::pack::pack_config::PackConfig;
use crate::models::repo::repo_config::RepoConfig;
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

struct Fixture {
    _tmp: TestTempDir,
    repo_path: PathBuf,
}

impl Fixture {
    fn new(key: &str) -> Self {
        let tmp = TestTempDir::new(key);
        let repo_path = tmp.path().join("repo");

        // Minimal repo config with one pack.
        let mut packs = HashSet::new();
        packs.insert("core".to_string());
        let repo_config = RepoConfig::new("repo".to_string(), "test".to_string(), packs);
        ServerRepoHandle::create(tmp.path(), repo_config).unwrap();

        // Minimal pack config, no required/optional addons declared.
        let pack_config = PackConfig::new(
            "core".to_string(),
            "test pack".to_string(),
            vec![],
            vec![],
            None,
        );
        pack_config.write_to_named(&repo_path, "core").unwrap();

        // Source pack addon dir with one addon containing a tiny file.
        let addon_dir = repo_path
            .join(get_pack_addon_directory_name("core"))
            .join("@addon1");
        fs::create_dir_all(addon_dir.join("sub")).unwrap();
        fs::write(addon_dir.join("file.txt"), b"hello").unwrap();
        fs::write(addon_dir.join("sub").join("nested.txt"), b"world").unwrap();

        Self {
            _tmp: tmp,
            repo_path,
        }
    }

    fn open(&self) -> ServerRepoHandle {
        ServerRepoHandle::open(&self.repo_path).unwrap()
    }

    // Absolute path to the fixture repo's `www` build-output dir (the fixture
    // repo lives under the system temp directory).
    fn www(&self) -> PathBuf {
        self.repo_path.join(WWW_DIR_NAME)
    }
}

fn opts(mode: BuildMode) -> BuildOptions {
    BuildOptions {
        mode,
        force_refresh: false,
    }
}

#[test]
fn build_pack_symlink_creates_links() {
    let fx = Fixture::new("pamm_build_symlink_creates_links");
    let server = fx.open();

    server
        .build_pack("core", opts(BuildMode::Symlink), &NoopProgress)
        .unwrap();

    let www_file = fx.www().join("core_pack_addons/@addon1/file.txt");
    let www_nested = fx.www().join("core_pack_addons/@addon1/sub/nested.txt");

    let md = fs::symlink_metadata(&www_file).unwrap();
    assert!(md.file_type().is_symlink());
    assert_eq!(fs::read(&www_file).unwrap(), b"hello");
    assert_eq!(fs::read(&www_nested).unwrap(), b"world");
}

#[test]
fn build_pack_copy_creates_files() {
    let fx = Fixture::new("pamm_build_copy_creates_files");
    let server = fx.open();

    server
        .build_pack("core", opts(BuildMode::Copy), &NoopProgress)
        .unwrap();

    let www_file = fx.www().join("core_pack_addons/@addon1/file.txt");
    let md = fs::symlink_metadata(&www_file).unwrap();
    assert!(md.file_type().is_file());
    assert!(!md.file_type().is_symlink());
    assert_eq!(fs::read(&www_file).unwrap(), b"hello");
}

#[test]
fn build_pack_writes_indexes_to_www_only() {
    let fx = Fixture::new("pamm_build_indexes_to_www_only");
    let server = fx.open();
    server
        .build_pack("core", opts(BuildMode::Symlink), &NoopProgress)
        .unwrap();

    let www_indexes = fx.www().join("core_pack_addons").join(INDEX_DIR_NAME);
    let www_checksum = www_indexes.join(ChecksumIndex::file_name());
    let www_addon_index = www_indexes.join("@addon1.index.pamm");
    assert!(www_checksum.is_file(), "checksum index should be in www");
    assert!(www_addon_index.is_file(), "addon index should be in www");

    // No indexes/ dir should appear next to the source.
    let source_indexes = fx.repo_path.join("core_pack_addons").join(INDEX_DIR_NAME);
    assert!(
        !source_indexes.exists(),
        "indexes/ should not exist next to the source files"
    );

    // Confirm the checksum index parses.
    let parsed = ChecksumIndex::read_from_known(&www_indexes).unwrap();
    assert!(parsed.checksums.contains_key("@addon1"));
}

#[test]
fn build_pack_removes_stale_addon_subtree() {
    let fx = Fixture::new("pamm_build_removes_stale_addon");
    let server = fx.open();

    // Pre-create a stale www tree with a phantom addon.
    let stale = fx.www().join("core_pack_addons/@phantom/file.txt");
    fs::create_dir_all(stale.parent().unwrap()).unwrap();
    fs::write(&stale, b"old").unwrap();

    server
        .build_pack("core", opts(BuildMode::Symlink), &NoopProgress)
        .unwrap();

    assert!(!stale.exists(), "phantom addon should be removed");
    assert!(fx.www().join("core_pack_addons/@addon1/file.txt").exists());
}

#[test]
fn build_pack_force_refresh_clears_cache() {
    let fx = Fixture::new("pamm_build_force_refresh_clears_cache");
    let server = fx.open();
    server
        .build_pack("core", opts(BuildMode::Symlink), &NoopProgress)
        .unwrap();

    // The cache dir should exist after the first build.
    let cache_dir = fx
        .repo_path
        .join("core_pack_addons")
        .join(CACHE_DB_DIR_NAME);
    assert!(cache_dir.is_dir());

    // Now force-refresh; the build should succeed (cache is wiped internally).
    server
        .build_pack(
            "core",
            BuildOptions {
                mode: BuildMode::Symlink,
                force_refresh: true,
            },
            &NoopProgress,
        )
        .unwrap();

    // After force_refresh, the cache dir exists again (sled recreates it).
    assert!(cache_dir.is_dir());
}

#[test]
fn build_repo_materializes_top_level_files() {
    let fx = Fixture::new("pamm_build_repo_top_level");
    let server = fx.open();
    server
        .build(opts(BuildMode::Symlink), NoopProgress)
        .unwrap();

    assert!(fx.www().join(RepoConfig::file_name()).exists());
    assert!(fx.www().join(PackConfig::get_file_name("core")).exists());
}

#[test]
fn build_pack_skips_dotcache_in_source() {
    let fx = Fixture::new("pamm_build_skips_dotcache");
    let server = fx.open();
    server
        .build_pack("core", opts(BuildMode::Symlink), &NoopProgress)
        .unwrap();

    // The sled cache dir should be at the source.
    let source_cache = fx
        .repo_path
        .join("core_pack_addons")
        .join(CACHE_DB_DIR_NAME);
    assert!(source_cache.exists(), "sled cache must remain at source");

    // It must NOT have been copied/linked into www.
    let www_cache = fx.www().join("core_pack_addons").join(CACHE_DB_DIR_NAME);
    assert!(!www_cache.exists(), ".cache/ must not appear in www output");
}

#[cfg(unix)]
#[test]
fn build_pack_symlink_target_is_relative() {
    let fx = Fixture::new("pamm_build_symlink_target_relative");
    let server = fx.open();
    server
        .build_pack("core", opts(BuildMode::Symlink), &NoopProgress)
        .unwrap();

    let link = fx.www().join("core_pack_addons/@addon1/file.txt");
    let target = fs::read_link(&link).unwrap();
    assert!(
        target.starts_with(".."),
        "symlink target should be relative ({:?})",
        target
    );
    assert!(!target.is_absolute(), "{:?} must be relative", target);
}
