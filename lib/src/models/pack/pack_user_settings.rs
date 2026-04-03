use crate::handle::externals::external_addon::ExternalAddon;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PackUserSettings {
    pub enabled_optionals: HashSet<String>,
    #[serde(default)]
    pub external_addons: HashSet<ExternalAddon>,
    #[serde(default)]
    pub launch_params: Vec<String>,
}
