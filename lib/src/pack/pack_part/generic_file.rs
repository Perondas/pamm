use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericFile {
    pub rel_path: String,
    pub length: u64,
    pub checksum: Vec<u8>
}