use crate::io::fs::fs_writable::FixedFsWritable;
use crate::io::fs::migration::migrations::v1_to_v2;
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
            1 => v1_to_v2::migrate_v1_to_v2(repo_path, repo_config)
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
