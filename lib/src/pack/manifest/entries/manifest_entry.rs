use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManifestEntry {
    pub(crate) name: String,
    pub(crate) checksum: Vec<u8>,
    pub(crate) kind: EntryKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum EntryKind {
    Folder(Vec<ManifestEntry>),
    File {
        last_modified: u64,
        length: u64,
        kind: FileKind,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum FileKind {
    Pbo {
        blob_offset: u64,
        parts: Vec<PBOPart>,
    },
    Generic,
    // TODO: maybe split large files into chunks?
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PBOPart {
    pub(crate) name: String,
    pub(crate) length: u32,
    pub(crate) checksum: Vec<u8>,
    pub(crate) start_offset: u64,
}
