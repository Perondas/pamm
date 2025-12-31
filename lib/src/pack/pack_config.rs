use crate::io::fs::util::clean_path::clean_path;
use crate::io::name_consts::get_pack_addon_directory_name;
use crate::named;
use crate::pack::addon::AddonSettings;
use crate::pack::server_info::ServerInfo;
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
            .map(|addon| addon_dir.join(&addon.0))
            .map(|p| p.canonicalize().unwrap())
            .map(clean_path)
            .collect()
    }
}
