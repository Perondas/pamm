use crate::manifest::pack_manifest::PackManifest;
use crate::pack::pack_config::PackConfig;

pub trait NamedFile {
    fn get_name(identifier: &str) -> String;
}

impl NamedFile for PackManifest {
    fn get_name(identifier: &str) -> String {
        format!("{}.pack.manifest.pamm", identifier)
    }
}

impl NamedFile for PackConfig {
    fn get_name(identifier: &str) -> String {
        format!("{}.pack.config.json", identifier)
    }
}
