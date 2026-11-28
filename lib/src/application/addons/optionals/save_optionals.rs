use crate::application::addons::optionals::OptionalAddon;
use crate::application::pack::read::get_pack::GetPack;
use crate::application::pack::write::save_pack_settings::SavePackSettings;
use crate::support::collect_packs::walk_packs;

pub trait SaveOptionals {
    /// Saves optionals and their status to the pack settings.
    fn save_optionals(
        &self,
        pack_name: &str,
        optionals: &[OptionalAddon],
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

impl<T> SaveOptionals for T
where
    T: GetPack + SavePackSettings + Sync,
{
    async fn save_optionals(
        &self,
        pack_name: &str,
        optionals: &[OptionalAddon],
    ) -> anyhow::Result<()> {
        let results = walk_packs(self, pack_name, |handle, name| async move {
            let (config, mut settings) = handle.get_pack_with_settings(&name).await?;

            settings.enabled_optionals = optionals
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

            Ok(handle.save_pack_settings(&name, &settings).await)
        })
        .await?;

        let errors: Vec<anyhow::Error> = results.into_iter().filter_map(|res| res.err()).collect();

        if !errors.is_empty() {
            Err(anyhow::anyhow!(
                "Failed to save optionals for some packs: {:#?}",
                errors
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::test::mock_handle::{MockHandle, MockHandleExt};

    #[tokio::test]
    async fn test_save_optionals() {
        let mut mock = MockHandle::new();

        mock.mock_pack("test_pack", None, &["@opt_addon"], &["@req_addon"], &[]);

        mock.expect_save_pack_settings()
            .withf(|pack_name, settings| {
                pack_name == "test_pack" && settings.enabled_optionals.contains("@opt_addon")
            })
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(()) }));

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

        mock.save_optionals("test_pack", &optionals).await.unwrap();
    }

    #[tokio::test]
    async fn test_save_optionals_with_parent() {
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
            .returning(|_, _| Box::pin(async { Ok(()) }));

        mock.expect_save_pack_settings()
            .withf(|name, settings| {
                name == "parent_pack"
                    && settings.enabled_optionals.contains("@parent_opt")
                    && !settings.enabled_optionals.contains("@child_opt")
            })
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(()) }));

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

        mock.save_optionals("child_pack", &optionals).await.unwrap();
    }
}
