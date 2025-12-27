use crate::index::index_node::IndexNode;
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
    pub fn to_names(self) -> HashSet<String> {
        self.addons.into_iter().map(|node| node.name).collect()
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
