use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PBOFile {
    pub rel_path: String,
    pub length: u64,
    pub checksum: Vec<u8>,
    pub parts: Vec<PBOPart>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PBOPart {
    pub rel_path: String,
    pub length: u64,
    pub checksum: Vec<u8>,
    pub start_offset: u64,
}