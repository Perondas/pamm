use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Addon {
    pub name: String,
    pub is_optional: bool,
}
