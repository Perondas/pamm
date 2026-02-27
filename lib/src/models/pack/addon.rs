use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct AddonSettings {
    pub is_optional: bool,
}
