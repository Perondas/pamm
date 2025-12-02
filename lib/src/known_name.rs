use crate::name_consts::{CONFIG_FILE_NAME, MANIFEST_FILE_NAME};
use crate::pack::config::pack_config::PackConfig;
use crate::pack::manifest::pack_manifest::PackManifest;

pub trait KnownName {
    fn known_name() -> &'static str;
}

impl KnownName for PackConfig {
    fn known_name() -> &'static str {
        CONFIG_FILE_NAME
    }
}

impl KnownName for PackManifest {
    fn known_name() -> &'static str {
        MANIFEST_FILE_NAME
    }
}
