use crate::io::name_consts::{pack_config_rel, pack_settings_rel};
use crate::models::index::index_node::IndexNode;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_user_settings::PackUserSettings;
use std::path::PathBuf;

pub trait NamedFile {
    fn get_file_name(identifier: &str) -> String;

    /// Repo-root-relative path of this file. Defaults to the bare file name;
    /// types that live inside per-pack folders override this. Also used to
    /// build download URLs, so it must match the published `www/` layout.
    fn get_rel_path(identifier: &str) -> PathBuf {
        Self::get_file_name(identifier).into()
    }
}

impl NamedFile for IndexNode {
    fn get_file_name(identifier: &str) -> String {
        format!("{}.index.pamm", identifier)
    }
}

impl NamedFile for PackConfig {
    /// Legacy (layout v1) root-level file name; current-layout paths come from
    /// `get_rel_path`.
    fn get_file_name(identifier: &str) -> String {
        format!("{}.pack.config.json", identifier)
    }

    fn get_rel_path(identifier: &str) -> PathBuf {
        pack_config_rel(identifier)
    }
}

impl NamedFile for PackUserSettings {
    /// Legacy (layout v1) root-level file name; current-layout paths come from
    /// `get_rel_path`.
    fn get_file_name(identifier: &str) -> String {
        format!("{}.pack.settings.json", identifier)
    }

    fn get_rel_path(identifier: &str) -> PathBuf {
        pack_settings_rel(identifier)
    }
}
