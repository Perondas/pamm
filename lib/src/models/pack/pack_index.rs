use crate::models::index::index_node::IndexNode;
use crate::io::fs::util::clean_path::clean_path;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackIndex {
    pub addons: Vec<IndexNode>,
    pub pack_name: String,
}

impl PackIndex {
    pub fn get_addon_names(&self) -> HashSet<&str> {
        self.addons.iter().map(|node| node.name.as_ref()).collect()
    }

    pub fn get_addon_paths(&self, base_path: &Path) -> Vec<String> {
        self.addons
            .iter()
            .map(|node| {
                base_path
                    .join(&node.name)
                    .canonicalize()
                    .expect("Failed to canonicalize path")
            })
            .map(clean_path)
            .collect()
    }
}
