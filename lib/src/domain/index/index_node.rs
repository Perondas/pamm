use crate::keyed;
use serde::{Deserialize, Serialize};

/// Length in bytes of the SHA-1 checksum that terminates every PBO file.
pub const PBO_CHECKSUM_LEN: u64 = 20;

keyed!(IndexNode);
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexNode {
    pub name: String,
    pub checksum: Vec<u8>,
    pub kind: NodeKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NodeKind {
    Folder(Vec<IndexNode>),
    File { length: u64, kind: FileKind },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileKind {
    Pbo {
        blob_start: u64,
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
