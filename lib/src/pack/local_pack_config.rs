use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalPackConfig {
    // TODO: add fields as needed
}

impl Default for LocalPackConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalPackConfig {
    pub fn new() -> Self {
        Self {}
    }
}
