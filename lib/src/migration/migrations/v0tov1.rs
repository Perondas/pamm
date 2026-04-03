use crate::handle::externals::external_addon::ExternalAddon;
use crate::migration::version_tag::VersionTag;
use crate::models::pack::pack_user_settings::OptionalAddonSetting;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn v0_to_v1(_path: &Path) -> anyhow::Result<VersionTag> {
    Ok(VersionTag::V1)
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct V0PackUserSettings {
    pub enabled_optionals: HashSet<String>,
    #[serde(default)]
    pub external_addons: HashSet<ExternalAddon>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V0RepoConfig {
    pub name: String,
    pub description: String,
    pub packs: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct V1PackUserSettings {
    pub enabled_optionals: HashMap<String, OptionalAddonSetting>,
    #[serde(default)]
    pub external_addons: HashSet<ExternalAddon>,
}
