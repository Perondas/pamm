use crate::domain::pack::pack_config::PackConfig;
use crate::domain::repo::repo_config::RepoConfig;
use crate::domain::repo::repo_user_settings::RepoUserSettings;
use crate::infrastructure::fs::files::file_names::fixed_file::FixedFile;
use anyhow::{Context, bail};
use log::{info, warn};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Migrates the repo from v3 to v4.
/// Packs can have multiple parents: `"parent": "X" | null` in every
/// `pack.config.json` (remote and local packs) becomes `"parents": ["X"] | []`.
pub fn migrate_v3_to_v4(repo_path: &Path, repo_config: &RepoConfig) -> anyhow::Result<()> {
    info!(
        "Starting migration from v3 to v4 for repo: {}",
        repo_config.name
    );

    let mut packs = repo_config.packs.iter().cloned().collect::<Vec<_>>();
    packs.extend(read_local_packs(repo_path)?);

    for pack in packs {
        let config_path = repo_path.join(&pack).join(PackConfig::file_name());

        if !config_path.is_file() {
            warn!(
                "Pack config for {} not found at {:?}, skipping",
                pack, config_path
            );
            continue;
        }

        migrate_pack_config(&config_path)
            .with_context(|| format!("migrating pack config of {}", pack))?;
    }

    info!(
        "Migration from v3 to v4 completed successfully for repo: {}",
        repo_config.name
    );

    Ok(())
}

/// Local packs live only in a client's `user.repo.settings.json`.
fn read_local_packs(repo_path: &Path) -> anyhow::Result<Vec<String>> {
    let settings_path = repo_path.join(RepoUserSettings::file_name());

    if !settings_path.is_file() {
        return Ok(Vec::new());
    }

    let settings: RepoUserSettings = serde_json::from_str(&fs::read_to_string(&settings_path)?)
        .with_context(|| format!("reading {:?}", settings_path))?;

    Ok(settings.local_packs)
}

fn migrate_pack_config(config_path: &Path) -> anyhow::Result<()> {
    let mut value: Value = serde_json::from_str(&fs::read_to_string(config_path)?)?;
    let object = value
        .as_object_mut()
        .context("pack config is not a JSON object")?;

    // Already migrated, e.g. by an interrupted earlier run.
    if object.contains_key("parents") {
        return Ok(());
    }

    let parents = match object.remove("parent") {
        None | Some(Value::Null) => vec![],
        Some(Value::String(parent)) => vec![parent],
        Some(other) => bail!("unexpected value for parent: {}", other),
    };

    object.insert("parents".to_string(), parents.into());

    // Round-trip through PackConfig so the result is known to load and keeps
    // the usual field order.
    let config: PackConfig = serde_json::from_value(value)?;
    fs::write(config_path, serde_json::to_string_pretty(&config)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::test::test_utils::TestTempDir;
    use std::collections::HashSet;
    use url::Url;

    fn write_pack_config(repo_path: &Path, pack: &str, parent: &str) {
        fs::create_dir_all(repo_path.join(pack)).unwrap();
        fs::write(
            repo_path.join(pack).join("pack.config.json"),
            format!(
                r#"{{
  "name": "{pack}",
  "description": "desc",
  "client_params": [],
  "parent": {parent},
  "addons": {{}},
  "customization": null,
  "enabled_cdlc": null
}}"#
            ),
        )
        .unwrap();
    }

    fn read_parents(repo_path: &Path, pack: &str) -> Vec<String> {
        let content = fs::read_to_string(repo_path.join(pack).join("pack.config.json")).unwrap();
        let config: PackConfig = serde_json::from_str(&content).unwrap();
        config.parents
    }

    fn repo_config(packs: &[&str]) -> RepoConfig {
        RepoConfig::new(
            "repo".to_string(),
            "desc".to_string(),
            packs.iter().map(|p| p.to_string()).collect::<HashSet<_>>(),
        )
    }

    #[test]
    fn converts_parent_into_parents() {
        let tmp = TestTempDir::new("pamm_migrate_v3_to_v4_parents");
        let repo_path = tmp.path();
        write_pack_config(repo_path, "base", "null");
        write_pack_config(repo_path, "main", r#""base""#);

        migrate_v3_to_v4(repo_path, &repo_config(&["base", "main"])).unwrap();

        assert!(read_parents(repo_path, "base").is_empty());
        assert_eq!(read_parents(repo_path, "main"), vec!["base"]);
        let content = fs::read_to_string(repo_path.join("main/pack.config.json")).unwrap();
        assert!(!content.contains("\"parent\""));
    }

    #[test]
    fn migrates_local_packs() {
        let tmp = TestTempDir::new("pamm_migrate_v3_to_v4_local");
        let repo_path = tmp.path();
        write_pack_config(repo_path, "base", "null");
        write_pack_config(repo_path, "mission", r#""base""#);

        let mut settings = RepoUserSettings::new(Url::parse("http://localhost/").unwrap());
        settings.local_packs.push("mission".to_string());
        fs::write(
            repo_path.join("user.repo.settings.json"),
            serde_json::to_string(&settings).unwrap(),
        )
        .unwrap();

        migrate_v3_to_v4(repo_path, &repo_config(&["base"])).unwrap();

        assert_eq!(read_parents(repo_path, "mission"), vec!["base"]);
    }

    #[test]
    fn is_idempotent() {
        let tmp = TestTempDir::new("pamm_migrate_v3_to_v4_idempotent");
        let repo_path = tmp.path();
        write_pack_config(repo_path, "base", "null");
        write_pack_config(repo_path, "main", r#""base""#);

        migrate_v3_to_v4(repo_path, &repo_config(&["base", "main"])).unwrap();
        migrate_v3_to_v4(repo_path, &repo_config(&["base", "main"])).unwrap();

        assert_eq!(read_parents(repo_path, "main"), vec!["base"]);
    }

    #[test]
    fn skips_packs_without_config() {
        let tmp = TestTempDir::new("pamm_migrate_v3_to_v4_missing");

        migrate_v3_to_v4(tmp.path(), &repo_config(&["absent"])).unwrap();
    }
}
