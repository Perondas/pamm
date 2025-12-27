use std::collections::HashSet;
use std::path::{Path, PathBuf};
use crate::index::index_node::IndexNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackIndex {
    pub addons: Vec<IndexNode>,
    pub pack_name: String,
}

impl PackIndex {
    pub fn to_names(self) -> HashSet<String> {
        self.addons
            .into_iter()
            .map(|node| node.name)
            .collect()
    }
    
    pub fn get_addon_paths(&self, base_path: &Path) -> Vec<String> {
        self.addons
            .iter()
            .map(|node| base_path.join(&node.name).canonicalize().expect("Failed to canonicalize path"))
            .map(clean_path)
            .collect()
    }
}


#[cfg(target_os = "windows")]
fn clean_path(path: PathBuf) -> String {
    path.to_str()
        .expect("mods must be UTF-8")
        .strip_prefix("\\\\?\\")
        .unwrap()
        .to_string()
}

#[cfg(target_os = "linux")]
fn clean_path(path: PathBuf) -> String {
    path.to_str().expect("mods must be UTF-8").to_string()
}