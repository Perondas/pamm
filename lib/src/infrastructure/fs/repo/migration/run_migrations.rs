use crate::domain::repo::repo_config::RepoConfig;
use crate::domain::repo::repo_version::{CURRENT_REPO_VERSION, RepoVersion};
use crate::infrastructure::fs::files::fs_writable::FixedFsWritable;
use crate::infrastructure::fs::repo::migration::migrations::v3_to_v4;
use anyhow::{Context, bail};
use log::info;
use std::path::Path;

/// The oldest layout version that can still be migrated to the current one.
const OLDEST_SUPPORTED_REPO_VERSION: u32 = 3;

/// Bring a local repo (server source or client) up to the current layout
/// version. The version is read from `version.pamm` at the repo root (absent
/// means v1); each pending migration runs in order and the file is updated
/// after every successful step, so an interrupted run resumes where it left
/// off. Repos already at the current version return immediately without
/// touching the filesystem.
pub(crate) async fn run_migrations(
    repo_path: &Path,
    repo_config: &RepoConfig,
) -> anyhow::Result<()> {
    let mut version = RepoVersion::read_or_v1(repo_path).await?.0;

    if version > CURRENT_REPO_VERSION {
        bail!(
            "Repo at {:?} has layout version {} but this pamm only supports up to {}; \
             update pamm",
            repo_path,
            version,
            CURRENT_REPO_VERSION
        );
    }

    if version < OLDEST_SUPPORTED_REPO_VERSION {
        bail!(
            "Repo at {:?} has layout version {} which is no longer supported; \
             the oldest version this pamm can migrate is {}",
            repo_path,
            version,
            OLDEST_SUPPORTED_REPO_VERSION
        );
    }

    while version < CURRENT_REPO_VERSION {
        match version {
            3 => {
                // Pack configs: single parent -> list of parents
                let (repo_path, repo_config) = (repo_path.to_path_buf(), repo_config.clone());
                tokio::task::spawn_blocking(move || {
                    v3_to_v4::migrate_v3_to_v4(&repo_path, &repo_config)
                })
                .await?
                .context("migrating repo layout from v3 to v4")?
            }
            v => unreachable!("no migration registered for version {}", v),
        }
        version += 1;
        RepoVersion(version)
            .write_fixed(repo_path)
            .await
            .context("writing version.pamm after migration")?;
        info!(
            "Repo at {:?} migrated to layout version {}",
            repo_path, version
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::fs::files::file_names::fixed_file::FixedFile;
    use crate::support::test::test_utils::TestTempDir;
    use std::collections::HashSet;
    use std::fs;

    fn repo_config(packs: &[&str]) -> RepoConfig {
        RepoConfig::new(
            "repo".to_string(),
            "desc".to_string(),
            packs.iter().map(|p| p.to_string()).collect::<HashSet<_>>(),
        )
    }

    async fn repo_at_version(tmp: &TestTempDir, version: u32) -> std::path::PathBuf {
        let repo_path = tmp.path().join("repo");
        fs::create_dir_all(&repo_path).unwrap();
        RepoVersion(version).write_fixed(&repo_path).await.unwrap();
        repo_path
    }

    #[tokio::test]
    async fn current_version_skips_migrations() {
        let tmp = TestTempDir::new("pamm_migrate_current_version");
        let repo_path = repo_at_version(&tmp, CURRENT_REPO_VERSION).await;

        // An old-style pack config must NOT be touched once the repo is marked
        // as current — the version file is authoritative.
        fs::create_dir_all(repo_path.join("core")).unwrap();
        fs::write(
            repo_path.join("core/pack.config.json"),
            br#"{"parent":null}"#,
        )
        .unwrap();

        run_migrations(&repo_path, &repo_config(&["core"]))
            .await
            .unwrap();

        assert_eq!(
            fs::read_to_string(repo_path.join("core/pack.config.json")).unwrap(),
            r#"{"parent":null}"#
        );
    }

    #[tokio::test]
    async fn migrates_v3_to_current() {
        let tmp = TestTempDir::new("pamm_migrate_v3_to_current");
        let repo_path = repo_at_version(&tmp, 3).await;

        run_migrations(&repo_path, &repo_config(&[])).await.unwrap();

        assert_eq!(
            RepoVersion::read_or_v1(&repo_path).await.unwrap(),
            RepoVersion::current()
        );
    }

    #[tokio::test]
    async fn newer_version_is_an_error() {
        let tmp = TestTempDir::new("pamm_migrate_newer_version");
        let repo_path = repo_at_version(&tmp, CURRENT_REPO_VERSION + 1).await;

        let err = run_migrations(&repo_path, &repo_config(&[]))
            .await
            .unwrap_err()
            .to_string();
        assert!(err.contains("update pamm"), "unexpected error: {}", err);
    }

    #[tokio::test]
    async fn unsupported_old_version_is_an_error() {
        let tmp = TestTempDir::new("pamm_migrate_unsupported_version");
        let repo_path = repo_at_version(&tmp, 2).await;

        let err = run_migrations(&repo_path, &repo_config(&[]))
            .await
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("no longer supported"),
            "unexpected error: {}",
            err
        );
    }

    #[tokio::test]
    async fn missing_version_file_is_unsupported() {
        let tmp = TestTempDir::new("pamm_migrate_missing_version");
        let repo_path = tmp.path().join("repo");
        fs::create_dir_all(&repo_path).unwrap();
        assert!(!repo_path.join(RepoVersion::file_name()).exists());

        let err = run_migrations(&repo_path, &repo_config(&[]))
            .await
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("no longer supported"),
            "unexpected error: {}",
            err
        );
    }
}
