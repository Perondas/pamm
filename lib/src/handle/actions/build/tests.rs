use crate::handle::actions::build::{BuildMode, BuildOptions};
use crate::handle::server_repo_handle::ServerRepoHandle;
use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::fs::fs_writable::FixedFsWritable;
use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::files::name_consts::{CACHE_DB_DIR_NAME, WWW_DIR_NAME};
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::models::index::checksum_index::ChecksumIndex;
use crate::models::pack::pack_config::PackConfig;
use crate::models::repo::repo_config::RepoConfig;
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

        // Minimal pack config, no required/optional addons declared. Lays out
        // the per-pack source folder: core/pack.config.json + core/addons/.
        let pack_config = PackConfig::new(
            "core".to_string(),
            "test pack".to_string(),
            vec![],
            vec![],
            None,
        );
        pack_config.init_source_on_fs(&repo_path).unwrap();

        // Source pack addon dir with one addon containing a tiny file.
        let addon_dir = repo_path.join("core/addons/@addon1");
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

    let www_file = fx.www().join("core/addons/@addon1/file.txt");
    let www_nested = fx.www().join("core/addons/@addon1/sub/nested.txt");

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

    let www_file = fx.www().join("core/addons/@addon1/file.txt");
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

    let www_indexes = fx.www().join("core/indexes");
    let www_checksum = www_indexes.join(ChecksumIndex::file_name());
    let www_addon_index = www_indexes.join("@addon1.index.pamm");
    assert!(www_checksum.is_file(), "checksum index should be in www");
    assert!(www_addon_index.is_file(), "addon index should be in www");

    // No indexes/ dir should appear next to the source.
    let source_indexes = fx.repo_path.join("core/indexes");
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
    let stale = fx.www().join("core/addons/@phantom/file.txt");
    fs::create_dir_all(stale.parent().unwrap()).unwrap();
    fs::write(&stale, b"old").unwrap();

    server
        .build_pack("core", opts(BuildMode::Symlink), &NoopProgress)
        .unwrap();

    assert!(!stale.exists(), "phantom addon should be removed");
    assert!(fx.www().join("core/addons/@addon1/file.txt").exists());
}

#[test]
fn build_pack_force_refresh_clears_cache() {
    let fx = Fixture::new("pamm_build_force_refresh_clears_cache");
    let server = fx.open();
    server
        .build_pack("core", opts(BuildMode::Symlink), &NoopProgress)
        .unwrap();

    // The cache dir should exist after the first build.
    let cache_dir = fx.repo_path.join("core/addons").join(CACHE_DB_DIR_NAME);
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
    assert!(fx.www().join(RepoVersion::file_name()).exists());
    assert!(fx.www().join("core/pack.config.json").exists());
    assert_eq!(
        RepoVersion::read_or_v1(&fx.www()).unwrap(),
        RepoVersion::current()
    );
}

#[test]
fn build_pack_skips_dotcache_in_source() {
    let fx = Fixture::new("pamm_build_skips_dotcache");
    let server = fx.open();
    server
        .build_pack("core", opts(BuildMode::Symlink), &NoopProgress)
        .unwrap();

    // The sled cache dir should be at the source.
    let source_cache = fx.repo_path.join("core/addons").join(CACHE_DB_DIR_NAME);
    assert!(source_cache.exists(), "sled cache must remain at source");

    // It must NOT have been copied/linked into www.
    let www_cache = fx.www().join("core/addons").join(CACHE_DB_DIR_NAME);
    assert!(!www_cache.exists(), ".cache/ must not appear in www output");
}

// A second build runs with the sled cache already present in the source addons
// dir — it must still be kept out of www and not be pruned into existence.
#[test]
fn rebuild_keeps_dotcache_out_of_www() {
    let fx = Fixture::new("pamm_rebuild_keeps_dotcache_out");
    let server = fx.open();

    server
        .build_pack("core", opts(BuildMode::Symlink), &NoopProgress)
        .unwrap();
    server
        .build_pack("core", opts(BuildMode::Symlink), &NoopProgress)
        .unwrap();

    let www_cache = fx.www().join("core/addons").join(CACHE_DB_DIR_NAME);
    assert!(
        !www_cache.exists(),
        ".cache/ must not appear in www output after a rebuild"
    );
    assert!(fx.www().join("core/addons/@addon1/file.txt").exists());
}

#[cfg(unix)]
#[test]
fn build_pack_symlink_target_is_relative() {
    let fx = Fixture::new("pamm_build_symlink_target_relative");
    let server = fx.open();
    server
        .build_pack("core", opts(BuildMode::Symlink), &NoopProgress)
        .unwrap();

    let link = fx.www().join("core/addons/@addon1/file.txt");
    let target = fs::read_link(&link).unwrap();
    assert!(
        target.starts_with(".."),
        "symlink target should be relative ({:?})",
        target
    );
    assert!(!target.is_absolute(), "{:?} must be relative", target);
}

#[cfg(unix)]
#[test]
fn build_pack_symlinks_point_into_per_pack_source() {
    let fx = Fixture::new("pamm_build_symlink_per_pack_target");
    let server = fx.open();
    server
        .build_pack("core", opts(BuildMode::Symlink), &NoopProgress)
        .unwrap();

    let link = fx.www().join("core/addons/@addon1/file.txt");
    assert_eq!(
        fs::read_link(&link).unwrap(),
        PathBuf::from("../../../../core/addons/@addon1/file.txt")
    );
    assert_eq!(fs::read(&link).unwrap(), b"hello", "symlink must resolve");
}

fn walk_relative(root: &PathBuf) -> Vec<String> {
    fn walk(root: &PathBuf, dir: &PathBuf, out: &mut Vec<String>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            out.push(
                path.strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            );
            // Don't follow symlinks: www files are symlinks into the source.
            if fs::symlink_metadata(&path).unwrap().is_dir() {
                walk(root, &path, out);
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out.sort();
    out
}

const EXPECTED_WWW_ENTRIES: &[&str] = &[
    "core",
    "core/addons",
    "core/addons/@addon1",
    "core/addons/@addon1/file.txt",
    "core/addons/@addon1/sub",
    "core/addons/@addon1/sub/nested.txt",
    "core/indexes",
    "core/indexes/@addon1.index.pamm",
    "core/indexes/checksum_index.pamm",
    "core/pack.config.json",
    "repo.config.json",
    "version.pamm",
];

// The exact www/ entry set is the client-facing contract for layout v2.
#[test]
fn build_produces_the_v2_www_shape_clients_expect() {
    let fx = Fixture::new("pamm_build_www_shape");
    let server = fx.open();
    server
        .build(opts(BuildMode::Symlink), NoopProgress)
        .unwrap();

    assert_eq!(walk_relative(&fx.www()), EXPECTED_WWW_ENTRIES);
}

// Building over a www tree produced by the legacy flat (v1) layout prunes the
// old entries so the published tree is exactly the v2 shape.
#[test]
fn build_prunes_legacy_v1_www_entries() {
    let fx = Fixture::new("pamm_build_prunes_v1_www");
    let server = fx.open();

    // Fake a leftover v1 www tree.
    let www = fx.www();
    fs::create_dir_all(www.join("core_pack_addons/@addon1")).unwrap();
    fs::create_dir_all(www.join("core_pack_addons/indexes")).unwrap();
    fs::write(www.join("core_pack_addons/@addon1/file.txt"), b"old").unwrap();
    fs::write(www.join("core.pack.config.json"), b"{}").unwrap();

    server
        .build(opts(BuildMode::Symlink), NoopProgress)
        .unwrap();

    assert_eq!(walk_relative(&fx.www()), EXPECTED_WWW_ENTRIES);
}

// Stale pack dirs (pack removed from repo.config.json) are pruned, while
// unrelated files/dirs the operator may keep in www are left alone.
#[test]
fn build_prunes_stale_pack_dirs_but_keeps_unrelated_entries() {
    let fx = Fixture::new("pamm_build_prunes_stale_pack_dirs");
    let server = fx.open();

    let www = fx.www();
    fs::create_dir_all(www.join("oldpack")).unwrap();
    fs::write(www.join("oldpack/pack.config.json"), b"{}").unwrap();
    fs::create_dir_all(www.join(".well-known")).unwrap();
    fs::write(www.join("robots.txt"), b"deny all").unwrap();

    server
        .build(opts(BuildMode::Symlink), NoopProgress)
        .unwrap();

    assert!(!www.join("oldpack").exists(), "stale pack dir is pruned");
    assert!(www.join(".well-known").is_dir(), "unrelated dir survives");
    assert!(www.join("robots.txt").is_file(), "unrelated file survives");
}

// End-to-end: opening a legacy flat (v1) repo migrates the source into
// per-pack folders, stamps version.pamm, and the subsequent build publishes
// the v2 www shape.
#[test]
fn open_migrates_v1_source_then_build_publishes_v2_www() {
    let tmp = TestTempDir::new("pamm_migrate_then_build");
    let repo_path = tmp.path().join("repo");

    let mut packs = HashSet::new();
    packs.insert("core".to_string());
    let repo_config = RepoConfig::new("repo".to_string(), "test".to_string(), packs);
    ServerRepoHandle::create(tmp.path(), repo_config).unwrap();

    // Turn the fresh repo into a v1 one: flat source files, no version.pamm.
    fs::remove_file(repo_path.join(RepoVersion::file_name())).unwrap();
    let pack_config = PackConfig::new(
        "core".to_string(),
        "test pack".to_string(),
        vec![],
        vec![],
        None,
    );
    pack_config.write_fixed(&repo_path).unwrap();
    let addon_dir = repo_path.join("core_pack_addons").join("@addon1");
    fs::create_dir_all(addon_dir.join("sub")).unwrap();
    fs::write(addon_dir.join("file.txt"), b"hello").unwrap();
    fs::write(addon_dir.join("sub").join("nested.txt"), b"world").unwrap();

    let server = ServerRepoHandle::open(&repo_path).unwrap();

    assert!(repo_path.join("core/pack.config.json").is_file());
    assert!(repo_path.join("core/addons/@addon1/file.txt").is_file());
    assert!(!repo_path.join("core.pack.config.json").exists());
    assert!(!repo_path.join("core_pack_addons").exists());
    assert_eq!(
        RepoVersion::read_or_v1(&repo_path).unwrap(),
        RepoVersion::current()
    );

    server
        .build(opts(BuildMode::Symlink), NoopProgress)
        .unwrap();

    let www = repo_path.join(WWW_DIR_NAME);
    assert_eq!(walk_relative(&www), EXPECTED_WWW_ENTRIES);
    assert_eq!(
        fs::read(www.join("core/addons/@addon1/file.txt")).unwrap(),
        b"hello"
    );
}
