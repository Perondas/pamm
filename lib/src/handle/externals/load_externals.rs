use crate::handle::externals::external_addon::ExternalAddon;
use crate::handle::reading::get_pack::GetPack;

pub trait LoadExternals {
    fn load_externals(&self, pack_name: &str) -> anyhow::Result<Vec<ExternalAddon>>;
}

impl<T> LoadExternals for T
where
    T: GetPack,
{
    fn load_externals(&self, pack_name: &str) -> anyhow::Result<Vec<ExternalAddon>> {
        let (_, settings) = self.get_pack_with_settings(pack_name)?;

        Ok(settings.external_addons.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handle::mock_handle::MockHandle;
    use crate::models::pack::pack_config::PackConfig;
    use crate::models::pack::pack_user_settings::PackUserSettings;

    #[test]
    fn test_load_externals() {
        let mut mock = MockHandle::new();

        let config = PackConfig::new(
            "test_pack".to_string(),
            "desc".to_string(),
            vec![],
            vec![],
            None,
        );

        let mut settings = PackUserSettings::default();
        let external = ExternalAddon {
            path: "/some/path".to_string(),
            name: Some("addon".to_string()),
            enabled: true,
        };
        settings.external_addons.insert(external.clone());

        mock.expect_get_pack_with_settings()
            .with(mockall::predicate::eq("test_pack"))
            .returning(move |_| Ok((config.clone(), settings.clone())));

        let mut externals = mock.load_externals("test_pack").unwrap();
        assert_eq!(externals.len(), 1);
        let ext = externals.pop().unwrap();
        assert_eq!(ext.path, "/some/path");
        assert_eq!(ext.name, Some("addon".to_string()));
        assert!(ext.enabled);
    }
}
