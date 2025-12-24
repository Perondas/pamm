use crate::index::index_node::IndexNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackIndex {
    pub addons: Vec<IndexNode>,
    pub pack_name: String,
}

impl PackIndex {
    pub fn to_map(self) -> std::collections::HashMap<String, Vec<u8>> {
        self.addons
            .into_iter()
            .map(|node| (node.name, node.checksum))
            .collect()
    }
}
