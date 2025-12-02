use crate::pack::manifest::entries::manifest_entry::ManifestEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PackManifest {
    pub required_addons: Vec<ManifestEntry>,
    pub optional_addons: Vec<ManifestEntry>,
    pub pack_checksum: Vec<u8>,
}
