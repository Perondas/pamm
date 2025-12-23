use serde::{Deserialize, Serialize};
use crate::index::index_node::{IndexNode, PBOPart};

#[derive(Debug)]
pub enum NodeDiff {
    Created(IndexNode),
    Deleted(String),
    Modified(NodeModification),
    None,
}

#[derive(Debug)]
pub struct NodeModification {
    pub name: String,
    pub kind: ModifiedNodeKind,
}

#[derive(Debug)]
pub enum ModifiedNodeKind {
    Folder(Vec<NodeDiff>),
    File {
        new_length: u64,
        target_checksum: Vec<u8>,
        modification: FileModification,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileModification {
    PBO {
        new_order: Vec<PBOPart>,
        required_checksums: Vec<Vec<u8>>,
        required_parts_size: u64,
        new_blob_offset: u64,
    },
    Generic,
}
