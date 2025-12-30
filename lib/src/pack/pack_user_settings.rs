use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackUserSettings {
    // TODO: add fields as needed
    pub enabled_optionals: Vec<String>,
}

impl Default for PackUserSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl PackUserSettings {
    pub fn new() -> Self {
        Self {
            enabled_optionals: Vec::new(),
        }
    }
}
