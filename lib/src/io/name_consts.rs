use std::path::{Path, PathBuf};

pub(crate) const CACHE_DB_DIR_NAME: &str = ".cache";
pub(crate) const INDEX_DIR_NAME: &str = "indexes";
pub(crate) const WWW_DIR_NAME: &str = "www";

// File/dir names inside a per-pack folder (`<repo>/<pack>/`). The pack folder
// itself carries the pack name, so these are fixed. Used everywhere since
// layout v2: server source, client repos, and the published `www/`.
pub(crate) const PACK_CONFIG_FILE_NAME: &str = "pack.config.json";
pub(crate) const PACK_SETTINGS_FILE_NAME: &str = "pack.settings.json";
pub(crate) const ADDONS_DIR_NAME: &str = "addons";

/// Repo-root-relative path of a pack's config file: `<pack>/pack.config.json`.
pub(crate) fn pack_config_rel(pack_name: &str) -> PathBuf {
    Path::new(pack_name).join(PACK_CONFIG_FILE_NAME)
}

/// Repo-root-relative path of a pack's user settings file:
/// `<pack>/pack.settings.json`.
pub(crate) fn pack_settings_rel(pack_name: &str) -> PathBuf {
    Path::new(pack_name).join(PACK_SETTINGS_FILE_NAME)
}

/// Repo-root-relative path of a pack's addon directory: `<pack>/addons`.
pub(crate) fn pack_addons_rel(pack_name: &str) -> PathBuf {
    Path::new(pack_name).join(ADDONS_DIR_NAME)
}

/// Repo-root-relative path of a pack's index directory: `<pack>/indexes`.
pub(crate) fn pack_indexes_rel(pack_name: &str) -> PathBuf {
    Path::new(pack_name).join(INDEX_DIR_NAME)
}

/// Legacy (layout v1) addon directory name at the repo root. Only referenced by
/// the v1 -> v2 migration and the build's stale-entry pruning.
pub(crate) fn get_pack_addon_directory_name(pack_name: &str) -> String {
    format!("{}_pack_addons", pack_name)
}
