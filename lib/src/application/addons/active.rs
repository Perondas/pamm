use crate::application::pack::read::get_pack::GetPack;
use crate::domain::addon::PackAddon;
use crate::domain::pack::pack_user_settings::PackUserSettings;

pub trait GetActiveAddons {
    /// Gets the addons that take part in launching the pack: the required
    /// addons of the pack and all of its ancestors, plus their enabled optionals
    /// when `include_optionals` is set. Own addons first, then each ancestor's
    /// in turn.
    fn get_active_addons(
        &self,
        pack_name: &str,
        include_optionals: bool,
    ) -> impl Future<Output = anyhow::Result<Vec<PackAddon>>> + Send;
}

impl<T> GetActiveAddons for T
where
    T: GetPack + Sync,
{
    async fn get_active_addons(
        &self,
        pack_name: &str,
        include_optionals: bool,
    ) -> anyhow::Result<Vec<PackAddon>> {
        let mut addons = Vec::new();
        let mut current = Some(pack_name.to_string());

        while let Some(pack_name) = current {
            // Without optionals the user settings are irrelevant, so don't
            // require them (the server has none).
            let (config, settings) = if include_optionals {
                self.get_pack_with_settings(&pack_name).await?
            } else {
                (
                    self.get_pack(&pack_name).await?,
                    PackUserSettings::default(),
                )
            };

            addons.extend(config.active_addons(&settings));

            current = config.parent;
        }

        log::debug!("Active addons for pack '{}': {:#?}", pack_name, addons);

        Ok(addons)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::test::mock_handle::{MockHandle, MockHandleExt};

    fn names(addons: &[PackAddon]) -> Vec<(&str, &str)> {
        let mut names = addons
            .iter()
            .map(|a| (a.pack_name.as_str(), a.name.as_str()))
            .collect::<Vec<_>>();
        names.sort();
        names
    }

    #[tokio::test]
    async fn includes_required_and_enabled_optionals_of_the_whole_chain() {
        let mut mock = MockHandle::new();
        mock.mock_pack(
            "child",
            Some("parent"),
            &["@child_opt_on", "@child_opt_off"],
            &["@child_req"],
            &["@child_opt_on"],
        );
        mock.mock_pack(
            "parent",
            None,
            &["@parent_opt_on"],
            &["@parent_req"],
            &["@parent_opt_on"],
        );

        let addons = mock.get_active_addons("child", true).await.unwrap();

        assert_eq!(
            names(&addons),
            vec![
                ("child", "@child_opt_on"),
                ("child", "@child_req"),
                ("parent", "@parent_opt_on"),
                ("parent", "@parent_req"),
            ]
        );
    }

    #[tokio::test]
    async fn leaves_out_optionals_when_asked() {
        let mut mock = MockHandle::new();
        mock.mock_pack(
            "child",
            Some("parent"),
            &["@child_opt"],
            &["@child_req"],
            &["@child_opt"],
        );
        mock.mock_pack(
            "parent",
            None,
            &["@parent_opt"],
            &["@parent_req"],
            &["@parent_opt"],
        );

        let addons = mock.get_active_addons("child", false).await.unwrap();

        assert_eq!(
            names(&addons),
            vec![("child", "@child_req"), ("parent", "@parent_req")]
        );
    }
}
