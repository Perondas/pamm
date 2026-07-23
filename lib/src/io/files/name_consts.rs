pub(crate) const CACHE_DB_DIR_NAME: &str = ".cache";
pub(crate) const INDEX_DIR_NAME: &str = "indexes";
pub(crate) const MEDIA_DIR_NAME: &str = "media";
pub(crate) const WWW_DIR_NAME: &str = "www";

// File/dir names inside a per-pack folder (`<repo>/<pack>/`). The pack folder
// itself carries the pack name, so these are fixed. Used everywhere since
// layout v2: server source, client repos, and the published `www/`.
pub(crate) const ADDONS_DIR_NAME: &str = "addons";

/// Legacy (layout v1) addon directory name at the repo root. Only referenced by
/// the v1 -> v2 migration and the build's stale-entry pruning.
pub(crate) fn get_pack_addon_directory_name(pack_name: &str) -> String {
    format!("{}_pack_addons", pack_name)
}
