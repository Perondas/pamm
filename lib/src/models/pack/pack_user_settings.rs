use crate::handle::externals::external_addon::ExternalAddon;
use crate::io::serialization::versioning::versioned::Versioned;
use crate::io::serialization::versioning::versioned_user_settings::VersionedPackUserSettings;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PackUserSettings {
    #[serde(default)]
    pub enabled_optionals: HashMap<String, OptionalAddonSetting>,
    #[serde(default)]
    pub external_addons: HashSet<ExternalAddon>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OptionalAddonSetting {
    pub is_enabled: bool,
    pub is_transitive_enabled: bool,
}

impl OptionalAddonSetting {
    pub fn should_be_loaded(&self) -> bool {
        self.is_enabled || self.is_transitive_enabled
    }
}

impl Versioned for PackUserSettings {
    type WrapperType = VersionedPackUserSettings;
}
