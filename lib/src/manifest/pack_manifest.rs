use crate::manifest::entries::manifest_entry::ManifestEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PackManifest {
    pub name: String,
    pub addons: Vec<ManifestEntry>,
    pub repo_checksum: Vec<u8>,
}
