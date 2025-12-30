use crate::io::fs::util::clean_path::clean_path;
use crate::io::name_consts::get_pack_addon_directory_name;
use crate::named;
use crate::pack::server_info::ServerInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

named!(PackConfig);
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackConfig {
    pub name: String,
    pub description: String,
    pub client_params: Vec<String>,
    pub servers: Vec<ServerInfo>,
    pub parent: Option<String>,
    pub(crate) addons: HashSet<String>,
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
            addons: HashSet::new(),
        }
    }

    pub fn with_addons(mut self, addons: HashSet<String>) -> Self {
        self.addons = addons;
        self
    }

    pub fn get_addon_paths(&self, base_path: &Path) -> Vec<String> {
        let addon_dir = base_path.join(get_pack_addon_directory_name(&self.name));

        self.addons
            .iter()
            .map(|addon| addon_dir.join(addon))
            .map(|p| p.canonicalize().unwrap())
            .map(clean_path)
            .collect()
    }
}
