use crate::manifest::entries::manifest_entry::ManifestEntry;
use crate::manifest::entries::manifest_entry::PBOPart;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum EntryDiff {
    Created(ManifestEntry),
    Deleted(String),
    Modified(EntryModification),
}

#[derive(Debug)]
pub struct EntryModification {
    pub name: String,
    pub kind: ModifiedEntryKind,
}

#[derive(Debug)]
pub enum ModifiedEntryKind {
    Folder(Vec<EntryDiff>),
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
        new_length: u64,
        blob_offset: u64,
    },
    Generic,
}
