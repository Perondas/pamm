use crate::pack::pack_part::part::PackPart;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Folder {
    pub name: String,
    pub checksum: Vec<u8>,
    pub children: Vec<PackPart>,
}
