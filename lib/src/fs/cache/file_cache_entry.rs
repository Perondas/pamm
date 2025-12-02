use crate::pack::manifest::entries::manifest_entry::ManifestEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileCacheEntry {
    pub last_modified: u64,
    pub length: u64,
    pub part: ManifestEntry,
}
