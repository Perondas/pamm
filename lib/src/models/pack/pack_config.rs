use crate::models::pack::addon::AddonSettings;
use crate::models::pack::pack_diff::PackDiff;
use crate::models::pack::pack_user_settings::PackUserSettings;
use crate::models::pack::server_info::ServerInfo;
use crate::named;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        user_settings
            .enabled_optionals
            .insert("optional_enabled".to_string());

        config.remove_disabled_optionals(&user_settings);

        assert!(config.addons.contains_key("required_addon"));
        assert!(config.addons.contains_key("optional_enabled"));
        assert!(!config.addons.contains_key("optional_disabled"));
    }

}
