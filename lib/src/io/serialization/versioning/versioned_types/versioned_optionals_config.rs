use crate::io::serialization::versioning::versioned::VersionedWrapper;
use crate::models::repo::optionals_config::OptionalsConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum VersionedOptionalsConfig {
    V1(OptionalsConfig),
}

impl VersionedWrapper<OptionalsConfig> for VersionedOptionalsConfig {
    fn get(self) -> OptionalsConfig {
        match self {
            VersionedOptionalsConfig::V1(config) => config,
        }
    }

    fn wrap(value: OptionalsConfig) -> Self {
        VersionedOptionalsConfig::V1(value)
    }
}
