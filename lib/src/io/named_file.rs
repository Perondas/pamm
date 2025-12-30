use crate::index::index_node::IndexNode;
use crate::pack::pack_config::PackConfig;
use crate::pack::pack_user_settings::PackUserSettings;

pub trait NamedFile {
    fn get_file_name(identifier: &str) -> String;
}

impl NamedFile for IndexNode {
    fn get_file_name(identifier: &str) -> String {
        format!("{}.index.pamm", identifier)
    }
}

impl NamedFile for PackConfig {
    fn get_file_name(identifier: &str) -> String {
        format!("{}.pack.config.json", identifier)
    }
}

impl NamedFile for PackUserSettings {
    fn get_file_name(identifier: &str) -> String {
        format!("{}.pack.settings.json", identifier)
    }
}
