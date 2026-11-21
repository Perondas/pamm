use crate::application::pack::read::get_pack::GetPack;
use crate::infrastructure::fs::files::util::clean_path::canonicalize_and_clean_path;
use std::collections::HashSet;

pub(in crate::application) trait GetExternalsPaths {
    /// Gets the paths to the pack's enabled external addons as absolute,
    /// canonicalized path strings (relative paths in the user settings are
    /// resolved against the process working directory).
    fn get_externals_paths(
        &self,
        pack_name: &str,
    ) -> impl Future<Output = anyhow::Result<HashSet<String>>> + Send;
}

impl<T> GetExternalsPaths for T
where
    T: GetPack + Sync,
{
    async fn get_externals_paths(&self, pack_name: &str) -> anyhow::Result<HashSet<String>> {
        let (_, settings) = self.get_pack_with_settings(pack_name).await?;

        settings
            .external_addons
            .iter()
            .filter(|addon| addon.enabled)
            .filter(|addon| addon.path.exists())
            .map(|addon| addon.path.to_owned())
            .map(canonicalize_and_clean_path)
            .collect::<anyhow::Result<HashSet<_>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::addons::externals::ExternalAddon;
    use crate::domain::pack::pack_config::PackConfig;
    use crate::domain::pack::pack_user_settings::PackUserSettings;
    use crate::infrastructure::fs::files::util::clean_path::canonicalize_and_clean_path;
    use crate::support::test::mock_handle::MockHandle;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_get_external_addon_paths() {
        let mut mock = MockHandle::new();

        let config = PackConfig::new(
            "test_pack".to_string(),
            "desc".to_string(),
            vec![],
            None,
            None,
        );

        let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let mut settings = PackUserSettings::default();
        let external_enabled = ExternalAddon {
            path: manifest_path.clone(),
            name: Some("enabled_addon".to_string()),
            enabled: true,
            found_in_workshop: false,
        };
        // Path does not matter so much, but it must be valid for `canonicalize`
        let external_disabled = ExternalAddon {
            path: manifest_path,
            name: Some("disabled_addon".to_string()),
            enabled: false,
            found_in_workshop: false,
        };

        settings.external_addons.insert(external_enabled);
        settings.external_addons.insert(external_disabled);

        mock.expect_get_pack_with_settings()
            .with(mockall::predicate::eq("test_pack"))
            .returning(move |_| Ok((config.clone(), settings.clone())));

        let paths = mock.get_externals_paths("test_pack").await.unwrap();

        assert_eq!(paths.len(), 1);
        let valid_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .canonicalize()
            .unwrap();
        assert_eq!(
            paths.iter().next().unwrap(),
            &canonicalize_and_clean_path(valid_path).unwrap()
        );
    }
}
