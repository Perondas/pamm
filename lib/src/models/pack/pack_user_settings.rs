use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackUserSettings {
    // TODO: add fields as needed
    pub enabled_optionals: HashSet<String>,
}

impl Default for PackUserSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl PackUserSettings {
    pub fn new() -> Self {
        Self {
            enabled_optionals: HashSet::new(),
        }
    }
}
