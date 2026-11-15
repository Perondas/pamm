use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Meant to provide a quick mapping of the addons and their current checksums to allow clients
/// to quickly verify their local files without having to download the entire index tree.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ChecksumIndex {
    pub(crate) checksums: HashMap<String, Vec<u8>>,
}
