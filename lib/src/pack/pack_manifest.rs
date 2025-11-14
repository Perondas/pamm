use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PackManifest {
    pub addons: Vec<String>,
    pub pack_checksum: Vec<u8>,
}
