use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub rel_path: String,
    pub checksum: Vec<u8>
}
