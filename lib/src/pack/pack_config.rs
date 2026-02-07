use crate::io::fs::util::clean_path::clean_path;
use crate::io::name_consts::get_pack_addon_directory_name;
use crate::named;
use crate::pack::addon::AddonSettings;
use crate::pack::pack_user_settings::PackUserSettings;
use crate::pack::server_info::ServerInfo;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

named!(PackConfig);
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackConfig {
    pub name: String,
    pub description: String,
    pub client_params: Vec<String>,
    pub servers: Vec<ServerInfo>,
    pub parent: Option<String>,
    pub(crate) addons: HashMap<String, AddonSettings>,
}

impl PackConfig {
    pub fn new(
        name: String,
        description: String,
        client_params: Vec<String>,
        servers: Vec<ServerInfo>,
        parent: Option<String>,
    ) -> Self {
        Self {
            name,
            description,
            client_params,
            servers,
            parent,
            addons: HashMap::new(),
        }
    }

    pub fn update_addons(mut self, addons: HashSet<String>) -> Self {
        self.addons = addons
            .into_iter()
            .map(|name| {
                let settings = self.addons.remove(&name).unwrap_or_default();
                (name, settings)
            })
            .collect();

        self
    }

    pub fn get_addon_paths(&self, base_path: &Path) -> Vec<String> {
        let addon_dir = base_path.join(get_pack_addon_directory_name(&self.name));

        self.addons
            .iter()
            .map(|addon| addon_dir.join(addon.0))
            .map(|p| {
                p.canonicalize()
                    .with_context(|| format!("Failed to canonicalize {}", p.display()))
                    .unwrap()
            })
            .map(clean_path)
            .collect()
    }

    pub fn remove_disabled_optionals(&mut self, user_settings: &PackUserSettings) {
        self.addons.retain(|name, settings| {
            if settings.is_optional {
                user_settings.enabled_optionals.contains(name)
            } else {
                true
            }
        });
    }
}
