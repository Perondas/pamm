use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenericFile {
    pub name: String,
    pub last_modified: u64,
    pub length: u64,
    pub checksum: Vec<u8>
}