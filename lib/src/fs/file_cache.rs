use crate::pack::pack_part::part::PackPart;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileCache {
    pub last_modified: u64,
    pub length: u64,
    pub part: PackPart,
}
