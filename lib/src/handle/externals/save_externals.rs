use crate::handle::externals::external_addon::ExternalAddon;
use crate::handle::reading::get_pack::GetPack;
use crate::handle::writing::save_pack_settings::SavePackSettings;

pub trait SaveExternals {
    fn save_externals(
        &self,
        pack_name: &str,
        externals: &[ExternalAddon],
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

impl<T> SaveExternals for T
where
    T: GetPack + SavePackSettings + Sync,
{
    async fn save_externals(
        &self,
        pack_name: &str,
        externals: &[ExternalAddon],
    ) -> anyhow::Result<()> {
        let (_, mut settings) = self.get_pack_with_settings(pack_name).await?;

        settings.external_addons = externals.iter().cloned().collect();

        self.save_pack_settings(pack_name, &settings).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handle::mock_handle::{MockHandle, MockHandleExt};
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_save_externals() {
        let mut mock = MockHandle::new();

        mock.mock_pack("test_pack", None, &[], &[], &[]);

        mock.expect_save_pack_settings()
            .withf(|pack_name, settings| {
                pack_name == "test_pack" && settings.external_addons.len() == 1
            })
            .times(1)
            .returning(|_, _| Ok(()));

        let external = ExternalAddon {
            path: PathBuf::from("/some/path/to/addon"),
            name: Some("addon".to_string()),
            enabled: true,
        };

        mock.save_externals("test_pack", &[external]).await.unwrap();
    }

    #[tokio::test]
    async fn test_save_externals_invalid_path() {
        let mut mock = MockHandle::new();

        mock.mock_pack("test_pack", None, &[], &[], &[]);

        let external = ExternalAddon::new("/path/that/never/will/exist");

        assert!(external.is_err());
        assert_eq!(
            external.unwrap_err().to_string(),
            "External addon path is not a directory: \"/path/that/never/will/exist\""
        );
    }

    #[tokio::test]
    async fn test_save_externals_resolves_name() {
        use crate::util::test_utils::TestTempDir;
        let temp_dir = TestTempDir::new("pamm_test_save_externals");
        let addon_dir = temp_dir.path().join("my_dummy_mod");
        std::fs::create_dir_all(&addon_dir).unwrap();

        // Write the meta file that resolver uses
        let mod_cpp_path = addon_dir.join("mod.cpp");
        std::fs::write(&mod_cpp_path, r#"name = "Dummy Mod";"#).unwrap();

        let external = ExternalAddon::new(&addon_dir)
            .expect("Failed to create ExternalAddon from test directory");

        assert_eq!(
            external.name.as_ref().expect("No name"),
            "Dummy Mod",
            "Expected the resolved name to match the content of mod.cpp"
        );
    }
}
