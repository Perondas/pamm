use crate::handle::externals::external_addon::ExternalAddon;
use crate::io::serialization::versioning::versioned::VersionedWrapper;
use crate::models::pack::pack_user_settings::{OptionalAddonSetting, PackUserSettings};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "version")]
pub(crate) enum VersionedPackUserSettings {
    V0(V0PackUserSettings),
    V1(PackUserSettings),
}

impl Default for VersionedPackUserSettings {
    fn default() -> Self {
        VersionedPackUserSettings::V1(Default::default())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct V0PackUserSettings {
    #[serde(default)]
    pub enabled_optionals: HashSet<String>,
    #[serde(default)]
    pub external_addons: HashSet<ExternalAddon>,
}

impl VersionedWrapper<PackUserSettings> for VersionedPackUserSettings {
    fn get(self) -> PackUserSettings {
        match self {
            VersionedPackUserSettings::V0(v0) => PackUserSettings {
                enabled_optionals: v0
                    .enabled_optionals
                    .into_iter()
                    .map(|s| {
                        (
                            s,
                            OptionalAddonSetting {
                                is_enabled: true,
                                is_transitive_enabled: false,
                            },
                        )
                    })
                    .collect(),
                external_addons: v0.external_addons,
            },
            VersionedPackUserSettings::V1(v1) => v1,
        }
    }

    fn wrap(value: PackUserSettings) -> Self {
        VersionedPackUserSettings::V1(value)
    }
}
