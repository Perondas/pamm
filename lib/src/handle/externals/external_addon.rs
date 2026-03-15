use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalAddon {
    pub path: String,
    pub name: Option<String>,
    pub enabled: bool,
}

impl ExternalAddon {
    pub fn new(path: String) -> Self {
        ExternalAddon {
            path,
            name: None,
            enabled: true,
        }
    }
}

impl PartialEq for ExternalAddon {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for ExternalAddon {}

impl Hash for ExternalAddon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.path.as_bytes());
    }
}
