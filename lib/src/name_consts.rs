pub(crate) const CACHE_DB_DIR_NAME: &str = ".cache";

pub(crate) fn get_pack_config_file_name(pack_name: &str) -> String {
    format!("{}.pack.config.json", pack_name)
}

pub(crate) fn get_pack_manifest_file_name(pack_name: &str) -> String {
    format!("{}.pack.manifest.pamm", pack_name)
}

pub(crate) fn get_pack_addon_directory_name(pack_name: &str) -> String {
    format!("{}_pack_addons", pack_name)
}
