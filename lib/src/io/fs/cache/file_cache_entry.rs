use serde::{Deserialize, Serialize};
use crate::index::index_node::IndexNode;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileCacheEntry {
    pub last_modified: u64,
    pub length: u64,
    pub index: IndexNode,
}
