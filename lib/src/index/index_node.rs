use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexNode {
    pub name: String,
    pub checksum: Vec<u8>,
    pub kind: NodeKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NodeKind {
    Folder(Vec<IndexNode>),
    File {
        last_modified: u64,
        length: u64,
        kind: FileKind,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileKind {
    Pbo {
        blob_offset: u64,
        parts: Vec<PBOPart>,
    },
    Generic,
    // TODO: maybe split large files into chunks?
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PBOPart {
    pub name: String,
    pub length: u32,
    pub checksum: Vec<u8>,
    pub start_offset: u64,
}
