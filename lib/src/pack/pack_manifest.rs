use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PackManifest {
    pub addons: Vec<(String, Vec<u8>)>,
    pub pack_checksum: Vec<u8>,
}
