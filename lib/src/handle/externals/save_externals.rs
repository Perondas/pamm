use crate::handle::externals::external_addon::ExternalAddon;
use crate::handle::reading::get_pack::GetPack;
use crate::handle::writing::save_pack_settings::SavePackSettings;
use std::collections::HashSet;

pub trait SaveExternals {
    fn save_externals(&self, pack_name: &str, externals: &[ExternalAddon]) -> anyhow::Result<()>;
}

impl<T> SaveExternals for T
where
    T: GetPack + SavePackSettings,
{
    fn save_externals(&self, pack_name: &str, externals: &[ExternalAddon]) -> anyhow::Result<()> {
        let (_, mut settings) = self.get_pack_with_settings(pack_name)?;

        settings.external_addons = externals
            .iter()
            .map(|e| e.to_owned())
            .map(|mut e| {
                e.resolve_name()?;
                Ok(e)
            })
            .collect::<anyhow::Result<HashSet<_>>>()?;

        self.save_pack_settings(pack_name, &settings)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handle::mock_handle::{MockHandle, MockHandleExt};

    #[test]
    fn test_save_externals() {
        let mut mock = MockHandle::new();

        mock.mock_pack("test_pack", None, &[], &[], &[]);

        mock.expect_save_pack_settings()
            .withf(|pack_name, settings| {
                pack_name == "test_pack" && settings.external_addons.len() == 1
            })
            .times(1)
            .returning(|_, _| Ok(()));

        let external = ExternalAddon {
            path: ".".to_string(),
            name: Some("addon".to_string()),
            enabled: true,
        };

        mock.save_externals("test_pack", &[external]).unwrap();
    }

    #[test]
    fn test_save_externals_invalid_path() {
        let mut mock = MockHandle::new();

        mock.mock_pack("test_pack", None, &[], &[], &[]);

        let external = ExternalAddon {
            path: "/path/that/never/will/exist".to_string(),
            name: None,
            enabled: true,
        };

        let result = mock.save_externals("test_pack", &[external]);
        assert!(
            result.is_err(),
            "Expected save_externals to fail due to invalid add-on path"
        );
    }

    #[test]
    fn test_save_externals_resolves_name() {
        use crate::util::test_utils::TestTempDir;
        let temp_dir = TestTempDir::new("pamm_test_save_externals");
        let addon_dir = temp_dir.path().join("my_dummy_mod");
        std::fs::create_dir_all(&addon_dir).unwrap();

        // Write the meta file that resolver uses
        let mod_cpp_path = addon_dir.join("mod.cpp");
        std::fs::write(&mod_cpp_path, r#"name = "Dummy Mod";"#).unwrap();

        let mut mock = MockHandle::new();
        mock.mock_pack("test_pack", None, &[], &[], &[]);

        mock.expect_save_pack_settings()
            .withf(|pack_name, settings| {
                if pack_name != "test_pack" || settings.external_addons.len() != 1 {
                    return false;
                }
                let resolved = settings.external_addons.iter().next().unwrap();
                resolved.name == Some("Dummy Mod".to_string())
            })
            .times(1)
            .returning(|_, _| Ok(()));

        let external = ExternalAddon {
            path: addon_dir.to_string_lossy().to_string(),
            name: None,
            enabled: true,
        };

        mock.save_externals("test_pack", &[external]).unwrap();
    }
}
