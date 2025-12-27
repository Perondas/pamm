use crate::index::index_node::IndexNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileCacheEntry {
    pub last_modified: u64,
    pub length: u64,
    pub index: IndexNode,
}
