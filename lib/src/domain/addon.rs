use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct AddonSettings {
    pub is_optional: bool,
    /// When set, the addon is not served by the repo. Clients are expected to
    /// subscribe to it on the Steam workshop, and it is loaded from there.
    #[serde(default)]
    pub workshop_id: Option<String>,
}

impl AddonSettings {
    /// Whether clients get this addon from the Steam workshop rather than the repo.
    pub fn is_workshop(&self) -> bool {
        self.workshop_id.is_some()
    }
}
