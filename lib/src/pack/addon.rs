use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AddonSettings {
    pub is_optional: bool,
}

impl Default for AddonSettings {
    fn default() -> Self {
        Self { is_optional: false }
    }
}
