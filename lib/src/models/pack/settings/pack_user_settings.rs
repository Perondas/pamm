use crate::models::pack::settings::external_addon::ExternalAddon;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PackUserSettings {
    pub enabled_optionals: HashSet<String>,
    pub external_addons: HashSet<ExternalAddon>,
}
