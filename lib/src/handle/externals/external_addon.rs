use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ExternalAddon {
    pub path: String,
    pub enabled: bool,
}
