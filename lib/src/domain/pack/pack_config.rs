use crate::domain::addon::{AddonSettings, PackAddon};
use crate::domain::cdlc::Cdlc;
use crate::domain::pack::pack_customization::PackCustomization;
use crate::domain::pack::pack_user_settings::PackUserSettings;
use crate::keyed;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

keyed!(PackConfig);
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackConfig {
    pub name: String,
    pub description: String,
    pub client_params: Vec<String>,
    pub parent: Option<String>,
    pub addons: HashMap<String, AddonSettings>,
    pub customization: Option<PackCustomization>,
    #[serde(default)]
    pub enabled_cdlc: Option<Cdlc>,
}

impl PackConfig {
    pub fn new(
        name: String,
        description: String,
        client_params: Vec<String>,
        parent: Option<String>,
        enabled_cdlc: Option<Cdlc>,
    ) -> Self {
        Self {
            name,
            description,
            client_params,
            parent,
            addons: HashMap::new(),
            customization: None,
            enabled_cdlc,
        }
    }

    /// The pack's own addons that take part in a launch: every required addon,
    /// plus the optionals enabled in `user_settings`. Inherited addons are not
    /// included.
    pub fn active_addons<'a>(
        &'a self,
        user_settings: &'a PackUserSettings,
    ) -> impl Iterator<Item = PackAddon> + 'a {
        self.addons
            .iter()
            .filter(|(name, settings)| {
                !settings.is_optional || user_settings.enabled_optionals.contains(*name)
            })
            .map(|(name, settings)| PackAddon {
                pack_name: self.name.clone(),
                name: name.clone(),
                settings: settings.clone(),
            })
    }

    /// The compat addon for the pack's CDLC, which stands in for the CDLC on
    /// machines that do not have it installed.
    pub fn cdlc_compat_addon(&self) -> Option<PackAddon> {
        self.enabled_cdlc.map(|cdlc| PackAddon {
            pack_name: self.name.clone(),
            name: format!("{} compat", cdlc.name()),
            settings: AddonSettings {
                workshop_id: Some(cdlc.compat_mod_id().to_string()),
                ..Default::default()
            },
        })
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
            Some("parent_pack".to_string()),
            None,
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
            None,
            None,
        );

        // Required addon
        config.addons.insert(
            "required_addon".to_string(),
            AddonSettings {
                is_optional: false,
                ..Default::default()
            },
        );

        // Optional addon (enabled)
        config.addons.insert(
            "optional_enabled".to_string(),
            AddonSettings {
                is_optional: true,
                ..Default::default()
            },
        );

        // Optional addon (disabled)
        config.addons.insert(
            "optional_disabled".to_string(),
            AddonSettings {
                is_optional: true,
                ..Default::default()
            },
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

    #[test]
    fn test_active_addons() {
        let mut config = PackConfig::new(
            "test_pack".to_string(),
            "desc".to_string(),
            vec![],
            None,
            None,
        );
        config
            .addons
            .insert("@required".to_string(), AddonSettings::default());
        config.addons.insert(
            "@workshop".to_string(),
            AddonSettings {
                workshop_id: Some("450814997".to_string()),
                ..Default::default()
            },
        );
        for name in ["@optional_enabled", "@optional_disabled"] {
            config.addons.insert(
                name.to_string(),
                AddonSettings {
                    is_optional: true,
                    ..Default::default()
                },
            );
        }

        let mut user_settings = PackUserSettings::default();
        user_settings
            .enabled_optionals
            .insert("@optional_enabled".to_string());

        let mut active = config
            .active_addons(&user_settings)
            .map(|addon| {
                assert_eq!(addon.pack_name, "test_pack");
                addon.name
            })
            .collect::<Vec<_>>();
        active.sort();

        assert_eq!(active, vec!["@optional_enabled", "@required", "@workshop"]);
    }

    #[test]
    fn test_cdlc_compat_addon() {
        let mut config = PackConfig::new(
            "test_pack".to_string(),
            "desc".to_string(),
            vec![],
            None,
            None,
        );
        assert_eq!(config.cdlc_compat_addon(), None);

        config.enabled_cdlc = Some(Cdlc::GlobalMobilization);
        let compat = config.cdlc_compat_addon().unwrap();

        assert_eq!(compat.pack_name, "test_pack");
        assert_eq!(
            compat.source(),
            crate::domain::addon::AddonSource::Workshop { id: "1776428269" }
        );
    }
}
