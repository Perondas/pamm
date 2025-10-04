use serde::{Deserialize, Serialize};
use crate::pack::pack_part::part::PackPart;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Folder {
    pub rel_path: String,
    pub checksum: Vec<u8>,
    pub children: Vec<PackPart>,
}
