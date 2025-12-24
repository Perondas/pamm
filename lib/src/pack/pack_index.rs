use crate::index::index_node::IndexNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackIndex {
    pub addons: Vec<IndexNode>,
    pub pack_name: String,
}
