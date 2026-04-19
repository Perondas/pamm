use crate::handle::optionals::optional_addon::OptionalAddon;
use crate::handle::reading::get_pack::GetPack;
use crate::handle::writing::save_pack_settings::SavePackSettings;

pub trait SaveOptionals {
    fn save_optionals(&self, pack_name: &str, optionals: &[OptionalAddon]) -> anyhow::Result<()>;
}

impl<T> SaveOptionals for T
where
    T: GetPack + SavePackSettings,
{
    fn save_optionals(&self, pack_name: &str, optionals: &[OptionalAddon]) -> anyhow::Result<()> {
        let (config, mut settings) = self.get_pack_with_settings(pack_name)?;

        let enabled = optionals
            .iter()
            .filter(|optional| optional.enabled)
            .filter(|optional| {
                config
                    .addons
                    .get(&optional.name)
                    .is_some_and(|addon| addon.is_optional)
            })
            .map(|optional| optional.name.to_owned())
            .collect();

        settings.enabled_optionals = enabled;

        self.save_pack_settings(pack_name, &settings)?;

        if let Some(parent) = &config.parent {
            self.save_optionals(parent, optionals)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handle::mock_handle::{MockHandle, MockHandleExt};

    #[test]
    fn test_save_optionals() {
        let mut mock = MockHandle::new();

        mock.mock_pack("test_pack", None, &["@opt_addon"], &["@req_addon"], &[]);

        mock.expect_save_pack_settings()
            .withf(|pack_name, settings| {
                pack_name == "test_pack" && settings.enabled_optionals.contains("@opt_addon")
            })
            .times(1)
            .returning(|_, _| Ok(()));

        let optionals = vec![
            OptionalAddon {
                name: "@opt_addon".to_string(),
                enabled: true,
            },
            OptionalAddon {
                name: "@req_addon".to_string(), // req_addon is ignored by checking `is_optional`
                enabled: true,
            },
        ];

        mock.save_optionals("test_pack", &optionals).unwrap();
    }

    #[test]
    fn test_save_optionals_with_parent() {
        let mut mock = MockHandle::new();

        mock.mock_pack("child_pack", Some("parent_pack"), &["@child_opt"], &[], &[]);

        mock.mock_pack("parent_pack", None, &["@parent_opt"], &[], &[]);

        mock.expect_save_pack_settings()
            .withf(|name, settings| {
                name == "child_pack"
                    && settings.enabled_optionals.contains("@child_opt")
                    && !settings.enabled_optionals.contains("@parent_opt")
            })
            .times(1)
            .returning(|_, _| Ok(()));

        mock.expect_save_pack_settings()
            .withf(|name, settings| {
                name == "parent_pack"
                    && settings.enabled_optionals.contains("@parent_opt")
                    && !settings.enabled_optionals.contains("@child_opt")
            })
            .times(1)
            .returning(|_, _| Ok(()));

        let optionals = vec![
            OptionalAddon {
                name: "@child_opt".to_string(),
                enabled: true,
            },
            OptionalAddon {
                name: "@parent_opt".to_string(),
                enabled: true,
            },
        ];

        mock.save_optionals("child_pack", &optionals).unwrap();
    }
}
