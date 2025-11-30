use crate::pack::pack_part::part::PackPart;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PackManifest {
    pub required_addons: Vec<PackPart>,
    pub optional_addons: Vec<PackPart>,
    pub pack_checksum: Vec<u8>,
}
