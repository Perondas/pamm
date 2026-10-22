use crate::models::repo::repo_config::RepoConfig;
use anyhow::bail;
use log::info;
use std::fs::create_dir_all;
use std::path::Path;

/// Migrates the repo from v2 to v3.
/// Only a change for server repos: adds the mp_missions folder to the pack folder
pub fn migrate_v2_to_v3(repo_path: &Path, repo_config: &RepoConfig) -> anyhow::Result<()> {
    info!(
        "Starting migration from v2 to v3 for repo: {}",
        repo_config.name
    );

    if !repo_path.join("server.config.json").exists() {
        info!(
            "Not in server repo, skipping migration for repo: {}",
            repo_config.name
        );
        return Ok(());
    }

    for pack in &repo_config.packs {
        let pack_path = repo_path.join(pack);

        if !pack_path.exists() {
            bail!(
                "Pack {} does not exist in repo {}, cannot migrate.",
                pack,
                repo_config.name
            );
        }

        let mp_missions_path = pack_path.join("mp_missions");
        create_dir_all(&mp_missions_path)?;
    }

    info!(
        "Migration from v2 to v3 completed successfully for repo: {}",
        repo_config.name
    );

    Ok(())
}
