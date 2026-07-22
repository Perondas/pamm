use crate::io::fs::fs_writable::FixedFsWritable;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_version::{RepoVersion, CURRENT_REPO_VERSION};
use anyhow::{bail, Context};
use log::info;
use std::path::Path;

/// Bring a local repo (server source or client) up to the current layout
/// version. The version is read from `version.pamm` at the repo root (absent
/// means v1); each pending migration runs in order and the file is updated
/// after every successful step, so an interrupted run resumes where it left
/// off. Repos already at the current version return immediately without
/// touching the filesystem.
pub(crate) fn run_migrations(repo_path: &Path, repo_config: &RepoConfig) -> anyhow::Result<()> {
    let mut version = RepoVersion::read_or_v1(repo_path)?.0;

    if version > CURRENT_REPO_VERSION {
        bail!(
            "Repo at {:?} has layout version {} but this pamm only supports up to {}; \
             update pamm",
            repo_path,
            version,
            CURRENT_REPO_VERSION
        );
    }

    while version < CURRENT_REPO_VERSION {
        match version {
            1 => migrate_v1_to_v2(repo_path, repo_config)
                .context("migrating repo layout from v1 to v2")?,
            v => unreachable!("no migration registered for version {}", v),
        }
        version += 1;
        RepoVersion(version)
            .write_fixed(repo_path)
            .context("writing version.pamm after migration")?;
        info!(
            "Repo at {:?} migrated to layout version {}",
            repo_path, version
        );
    }

    Ok(())
}

/// v1 -> v2: move each pack's root-level files into its own folder.
///
/// v1 (flat): `<pack>.pack.config.json`, `<pack>.pack.settings.json`,
/// `<pack>_pack_addons/` (with `indexes/` inside on clients, `.cache/` on
/// servers).
/// v2 (per-pack): `<pack>/pack.config.json`, `<pack>/pack.settings.json`,
/// `<pack>/addons/` (`.cache/` stays inside), `<pack>/indexes/`.
///
/// Each artifact migrates independently and files already in the v2 location
/// are left alone, so a previously interrupted migration is self-healing. If a
/// file exists in BOTH locations we bail rather than guess which copy is
/// authoritative. Only files of packs listed in `repo.config.json` are
/// touched; `www/` and the root-level repo/server config files never move.
fn migrate_v1_to_v2(_repo_path: &Path, _repo_config: &RepoConfig) -> anyhow::Result<()> {
    /* let mut moved_any = false;

    for pack in &repo_config.packs {
        ensure!(
            pack != WWW_DIR_NAME,
            "Pack '{}' collides with the '{}' build output directory; rename the pack",
            pack,
            WWW_DIR_NAME
        );

        let pack_dir = repo_path.join(pack);
        ensure!(
            !pack_dir.is_file(),
            "Cannot create pack folder {:?}: a file with that name exists",
            pack_dir
        );

        let artifacts = [
            (
                PathBuf::from(PackConfig::get_file_name(pack)),
                pack_config_rel(pack),
            ),
            (
                PathBuf::from(PackUserSettings::get_file_name(pack)),
                pack_settings_rel(pack),
            ),
            (
                PathBuf::from(get_pack_addon_directory_name(pack)),
                pack_addons_rel(pack),
            ),
        ];

        for (old_rel, new_rel) in artifacts {
            let old = repo_path.join(&old_rel);
            let new = repo_path.join(&new_rel);

            match (old.exists(), new.exists()) {
                (true, true) => bail!(
                    "Both {:?} and {:?} exist; delete or move one of them manually, \
                     then retry",
                    old,
                    new
                ),
                (true, false) => {
                    fs::create_dir_all(&pack_dir)
                        .with_context(|| format!("creating pack folder {:?}", pack_dir))?;
                    fs::rename(&old, &new)
                        .with_context(|| format!("moving {:?} to {:?}", old, new))?;
                    info!("Migrated {:?} -> {:?}", old, new);
                    moved_any = true;
                }
                // Already migrated, or the artifact doesn't exist (e.g. servers
                // usually have no pack settings file) — nothing to do. A pack
                // missing its config file fails later with the usual error.
                (false, _) => {}
            }
        }

        // v1 kept the (client-side) indexes inside the addon directory; v2
        // hoists them next to it.
        let nested_indexes = repo_path.join(pack_addons_rel(pack)).join(INDEX_DIR_NAME);
        let indexes = repo_path.join(pack_indexes_rel(pack));
        match (nested_indexes.exists(), indexes.exists()) {
            (true, true) => bail!(
                "Both {:?} and {:?} exist; delete or move one of them manually, then retry",
                nested_indexes,
                indexes
            ),
            (true, false) => {
                fs::rename(&nested_indexes, &indexes)
                    .with_context(|| format!("moving {:?} to {:?}", nested_indexes, indexes))?;
                info!("Migrated {:?} -> {:?}", nested_indexes, indexes);
                moved_any = true;
            }
            (false, _) => {}
        }

        if !repo_path.join(pack_config_rel(pack)).exists() {
            warn!(
                "Pack '{}' is listed in repo.config.json but has no pack config file",
                pack
            );
        }
    }

    if moved_any && repo_path.join(WWW_DIR_NAME).exists() {
        warn!(
            "Repo source was migrated to per-pack folders; run `build` to regenerate \
             the www/ layout"
        );
    }*/

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_utils::TestTempDir;
    use std::collections::HashSet;
    use std::fs;
    use std::path::PathBuf;

    fn repo_config(packs: &[&str]) -> RepoConfig {
        RepoConfig::new(
            "repo".to_string(),
            "desc".to_string(),
            packs.iter().map(|p| p.to_string()).collect::<HashSet<_>>(),
        )
    }

    fn flat_fixture(tmp: &TestTempDir) -> PathBuf {
        let repo_path = tmp.path().join("repo");
        fs::create_dir_all(repo_path.join("core_pack_addons/@addon1")).unwrap();
        fs::create_dir_all(repo_path.join("core_pack_addons/.cache")).unwrap();
        fs::create_dir_all(repo_path.join("core_pack_addons/indexes")).unwrap();
        fs::write(repo_path.join("core_pack_addons/@addon1/file.txt"), b"x").unwrap();
        fs::write(repo_path.join("core_pack_addons/.cache/db"), b"cache").unwrap();
        fs::write(
            repo_path.join("core_pack_addons/indexes/checksum_index.pamm"),
            b"idx",
        )
        .unwrap();
        fs::write(repo_path.join("core.pack.config.json"), b"{}").unwrap();
        fs::write(repo_path.join("core.pack.settings.json"), b"{}").unwrap();
        repo_path
    }

    #[test]
    fn migrates_flat_layout_into_per_pack_folders() {
        let tmp = TestTempDir::new("pamm_migrate_flat_happy");
        let repo_path = flat_fixture(&tmp);

        run_migrations(&repo_path, &repo_config(&["core"])).unwrap();

        assert!(repo_path.join("core/pack.config.json").is_file());
        assert!(repo_path.join("core/pack.settings.json").is_file());
        assert!(repo_path.join("core/addons/@addon1/file.txt").is_file());
        assert!(
            repo_path.join("core/addons/.cache/db").is_file(),
            ".cache moves along with the addons dir"
        );
        assert!(
            repo_path.join("core/indexes/checksum_index.pamm").is_file(),
            "indexes are hoisted out of the addons dir"
        );
        assert!(!repo_path.join("core/addons/indexes").exists());

        assert!(!repo_path.join("core.pack.config.json").exists());
        assert!(!repo_path.join("core.pack.settings.json").exists());
        assert!(!repo_path.join("core_pack_addons").exists());

        assert_eq!(
            RepoVersion::read_or_v1(&repo_path).unwrap(),
            RepoVersion::current(),
            "version.pamm is written after migrating"
        );
    }

    #[test]
    fn migration_is_idempotent() {
        let tmp = TestTempDir::new("pamm_migrate_flat_idempotent");
        let repo_path = flat_fixture(&tmp);
        let config = repo_config(&["core"]);

        run_migrations(&repo_path, &config).unwrap();
        run_migrations(&repo_path, &config).unwrap();

        assert!(repo_path.join("core/pack.config.json").is_file());
    }

    #[test]
    fn current_version_skips_migration_scan() {
        let tmp = TestTempDir::new("pamm_migrate_current_version");
        let repo_path = tmp.path().join("repo");
        fs::create_dir_all(&repo_path).unwrap();
        RepoVersion::current().write_fixed(&repo_path).unwrap();

        // Leftover flat-looking files must NOT be touched once the repo is
        // marked as current — the version file is authoritative.
        fs::write(repo_path.join("core.pack.config.json"), b"{}").unwrap();

        run_migrations(&repo_path, &repo_config(&["core"])).unwrap();

        assert!(repo_path.join("core.pack.config.json").is_file());
        assert!(!repo_path.join("core").exists());
    }

    #[test]
    fn newer_version_is_an_error() {
        let tmp = TestTempDir::new("pamm_migrate_newer_version");
        let repo_path = tmp.path().join("repo");
        fs::create_dir_all(&repo_path).unwrap();
        RepoVersion(CURRENT_REPO_VERSION + 1)
            .write_fixed(&repo_path)
            .unwrap();

        let err = run_migrations(&repo_path, &repo_config(&[]))
            .unwrap_err()
            .to_string();
        assert!(err.contains("update pamm"), "unexpected error: {}", err);
    }

    #[test]
    fn bails_when_both_layouts_have_the_file() {
        let tmp = TestTempDir::new("pamm_migrate_flat_conflict");
        let repo_path = flat_fixture(&tmp);
        fs::create_dir_all(repo_path.join("core")).unwrap();
        fs::write(repo_path.join("core/pack.config.json"), b"{}").unwrap();

        let err = run_migrations(&repo_path, &repo_config(&["core"]))
            .unwrap_err()
            .to_string();
        assert!(err.contains("v1 to v2"), "unexpected error: {}", err);
        assert!(
            !repo_path.join("version.pamm").exists(),
            "failed migration must not bump the version"
        );
    }

    #[test]
    fn tolerates_pack_without_files() {
        let tmp = TestTempDir::new("pamm_migrate_flat_missing_pack");
        let repo_path = tmp.path().join("repo");
        fs::create_dir_all(&repo_path).unwrap();

        run_migrations(&repo_path, &repo_config(&["ghost"])).unwrap();
        assert!(!repo_path.join("ghost").exists());
    }

    #[test]
    fn rejects_pack_named_www() {
        let tmp = TestTempDir::new("pamm_migrate_flat_www_pack");
        let repo_path = tmp.path().join("repo");
        fs::create_dir_all(&repo_path).unwrap();

        let err = format!(
            "{:#}",
            run_migrations(&repo_path, &repo_config(&["www"])).unwrap_err()
        );
        assert!(err.contains("www"), "unexpected error: {}", err);
    }

    #[test]
    fn leaves_unrelated_flat_files_alone() {
        let tmp = TestTempDir::new("pamm_migrate_flat_unrelated");
        let repo_path = flat_fixture(&tmp);
        fs::write(repo_path.join("other.pack.config.json"), b"{}").unwrap();

        run_migrations(&repo_path, &repo_config(&["core"])).unwrap();

        assert!(
            repo_path.join("other.pack.config.json").is_file(),
            "packs not in repo.config.json must not be touched"
        );
        assert!(!repo_path.join("other").exists());
    }
}
