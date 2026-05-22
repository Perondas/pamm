pub(crate) const CACHE_DB_DIR_NAME: &str = ".cache";
pub(crate) const INDEX_DIR_NAME: &str = "indexes";
pub(crate) const WWW_DIR_NAME: &str = "www";

pub(crate) fn get_pack_addon_directory_name(pack_name: &str) -> String {
    format!("{}_pack_addons", pack_name)
}
