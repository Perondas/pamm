use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ExternalAddon {
    pub path: String,
    pub name: Option<String>,
    pub enabled: bool,
}
