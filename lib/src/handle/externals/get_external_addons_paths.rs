use crate::handle::reading::get_pack::GetPack;
use crate::io::fs::util::clean_path::canonicalize_and_clean_path;
use std::path::PathBuf;

impl<T> GetExternalAddonsPaths for T
where
    T: GetPack,
{
    fn get_external_addon_paths(&self, pack_name: &str) -> anyhow::Result<Vec<String>> {
        let (_, settings) = self.get_pack_with_settings(pack_name)?;

        settings
            .external_addons
            .iter()
            .filter(|addon| addon.enabled)
            .map(|addon| PathBuf::from(addon.path.to_owned()))
            .map(canonicalize_and_clean_path)
            .collect::<anyhow::Result<Vec<_>>>()
    }
}

pub(in crate::handle) trait GetExternalAddonsPaths {
    fn get_external_addon_paths(&self, pack_name: &str) -> anyhow::Result<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handle::externals::external_addon::ExternalAddon;
    use crate::handle::mock_handle::MockHandle;
    use crate::io::fs::util::clean_path::canonicalize_and_clean_path;
    use crate::models::pack::pack_config::PackConfig;
    use crate::models::pack::pack_user_settings::PackUserSettings;

    #[test]
    fn test_get_external_addon_paths() {
        let mut mock = MockHandle::new();

        let config = PackConfig::new(
            "test_pack".to_string(),
            "desc".to_string(),
            vec![],
            vec![],
            None,
        );

        let mut settings = PackUserSettings::default();
        let external_enabled = ExternalAddon {
            path: env!("CARGO_MANIFEST_DIR").to_string(),
            name: Some("enabled_addon".to_string()),
            enabled: true,
        };
        // Path does not matter so much, but it must be valid for `canonicalize`
        let external_disabled = ExternalAddon {
            path: env!("CARGO_MANIFEST_DIR").to_string(),
            name: Some("disabled_addon".to_string()),
            enabled: false,
        };

        settings.external_addons.insert(external_enabled);
        settings.external_addons.insert(external_disabled);

        mock.expect_get_pack_with_settings()
            .with(mockall::predicate::eq("test_pack"))
            .returning(move |_| Ok((config.clone(), settings.clone())));

        let paths = mock.get_external_addon_paths("test_pack").unwrap();

        assert_eq!(paths.len(), 1);
        let valid_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .canonicalize()
            .unwrap();
        assert_eq!(paths[0], canonicalize_and_clean_path(valid_path).unwrap());
    }
}
