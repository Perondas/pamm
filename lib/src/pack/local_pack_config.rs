use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalPackConfig {
    // TODO: add fields as needed
}

impl LocalPackConfig {
    pub fn new() -> Self {
        Self {}
    }
}
