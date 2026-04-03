use crate::handle::externals::external_addon::ExternalAddon;
use crate::io::serialization::versioning::versioned::Versioned;
use crate::io::serialization::versioning::versioned_types::versioned_user_settings::VersionedPackUserSettings;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PackUserSettings {
    #[serde(default)]
    pub external_addons: HashSet<ExternalAddon>,
    // TODO: Add Launch args
}

impl Versioned for PackUserSettings {
    type WrapperType = VersionedPackUserSettings;
}
