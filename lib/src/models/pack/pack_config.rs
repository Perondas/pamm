use crate::io::fs::util::clean_path::clean_path;
use crate::io::name_consts::get_pack_addon_directory_name;
use crate::models::pack::addon::AddonSettings;
use crate::models::pack::pack_diff::PackDiff;
use crate::models::pack::pack_user_settings::PackUserSettings;
use crate::models::pack::server_info::ServerInfo;
use crate::named;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

named!(PackConfig);
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackConfig {
    pub name: String,
    pub description: String,
    pub client_params: Vec<String>,
    pub servers: Vec<ServerInfo>,
    pub parent: Option<String>,
    pub addons: HashMap<String, AddonSettings>,
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

    pub fn update_addons(mut self, diff: &PackDiff) -> Self {
        let addons = diff.target_index.get_addon_names();
        self.addons = addons
            .into_iter()
            .map(|name| {
                let settings = self.addons.remove(name).unwrap_or_default();
                (name.to_string(), settings)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_new_pack_config() {
        let config = PackConfig::new(
            "test_pack".to_string(),
            "A test pack".to_string(),
            vec!["-mod=1".to_string()],
            vec![],
            Some("parent_pack".to_string()),
        );

        assert_eq!(config.name, "test_pack");
        assert_eq!(config.description, "A test pack");
        assert_eq!(config.client_params, vec!["-mod=1".to_string()]);
        assert_eq!(config.parent, Some("parent_pack".to_string()));
        assert!(config.addons.is_empty());
    }

    #[test]
    fn test_remove_disabled_optionals() {
        let mut config = PackConfig::new(
            "test_pack".to_string(),
            "desc".to_string(),
            vec![],
            vec![],
            None,
        );

        // Required addon
        config.addons.insert(
            "required_addon".to_string(),
            AddonSettings { is_optional: false },
        );

        // Optional addon (enabled)
        config.addons.insert(
            "optional_enabled".to_string(),
            AddonSettings { is_optional: true },
        );

        // Optional addon (disabled)
        config.addons.insert(
            "optional_disabled".to_string(),
            AddonSettings { is_optional: true },
        );

        let mut user_settings = PackUserSettings::default();
        user_settings.enabled_optionals.insert("optional_enabled".to_string());

        config.remove_disabled_optionals(&user_settings);

        assert!(config.addons.contains_key("required_addon"));
        assert!(config.addons.contains_key("optional_enabled"));
        assert!(!config.addons.contains_key("optional_disabled"));
    }

    #[test]
    fn test_get_addon_paths() {
        use crate::util::test_utils::TestTempDir;

        let mut config = PackConfig::new(
            "test_pack_for_paths".to_string(),
            "desc".to_string(),
            vec![],
            vec![],
            None,
        );

        config.addons.insert(
            "@addon1".to_string(),
            AddonSettings { is_optional: false },
        );
        config.addons.insert(
            "@addon2".to_string(),
            AddonSettings { is_optional: true },
        );

        let temp_dir = std::env::temp_dir().join(format!("pamm_test_addon_paths_{}", std::process::id()));
        let _cleanup = TestTempDir::new(temp_dir.clone());
        let addon_dir = temp_dir.join("test_pack_for_paths_pack_addons");

        std::fs::create_dir_all(addon_dir.join("@addon1")).unwrap();
        std::fs::create_dir_all(addon_dir.join("@addon2")).unwrap();

        let mut paths = config.get_addon_paths(&temp_dir);
        paths.sort();

        assert_eq!(paths.len(), 2);
        assert!(paths.iter().any(|p| p.ends_with("@addon1") || p.ends_with("@addon1/")));
        assert!(paths.iter().any(|p| p.ends_with("@addon2") || p.ends_with("@addon2/")));

        let empty_config = PackConfig::new(
            "test_empty_pack_paths".to_string(),
            "desc".to_string(),
            vec![],
            vec![],
            None,
        );
        assert!(empty_config.get_addon_paths(&temp_dir).is_empty());
    }
}
