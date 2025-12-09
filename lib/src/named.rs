use crate::manifest::pack_manifest::PackManifest;
use crate::name_consts::{get_pack_config_file_name, get_pack_manifest_file_name};
use crate::pack::pack_config::PackConfig;

pub trait Named {
    fn get_name(identifier: &str) -> String;
}

impl Named for PackConfig {
    fn get_name(identifier: &str) -> String {
        get_pack_config_file_name(identifier)
    }
}

impl Named for PackManifest {
    fn get_name(identifier: &str) -> String {
        get_pack_manifest_file_name(identifier)
    }
}
