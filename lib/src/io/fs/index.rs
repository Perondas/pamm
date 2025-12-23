use crate::index::index_node::IndexNode;
use crate::io::fs::fs_readable::NamedFSReadable;
use crate::io::name_consts::INDEX_FOLDER;
use anyhow::Result;
use std::path::Path;

pub fn get_stored_index(addon_folder_path: &Path, addon_name: &str) -> Result<Option<IndexNode>> {
    let indexes_folder = addon_folder_path.join(INDEX_FOLDER);
    IndexNode::read_from_named(&indexes_folder, addon_name)
}
