use crate::application::pack::read::get_pack::GetPack;
use crate::domain::addon::PackAddon;
use crate::domain::pack::pack_config::PackConfig;
use crate::domain::pack::pack_user_settings::PackUserSettings;
use crate::platform::steam::has_cdlc_installed::has_cdlc_installed;
use crate::support::collect_packs::walk_packs;
use anyhow::Context;

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

    /// Like [`GetActiveAddons::get_active_addons`], plus the compat for the
    /// pack's CDLC when this machine does not have the CDLC installed. These are
    /// the addons launching the pack on this machine needs.
    fn get_launch_addons(
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
        let addons = walk_packs(self, pack_name, |handle, name| async move {
            // Without optionals the user settings are irrelevant, so don't
            // require them (the server has none).
            let (config, settings) = if include_optionals {
                handle.get_pack_with_settings(&name).await?
            } else {
                (handle.get_pack(&name).await?, PackUserSettings::default())
            };

            Ok(config.active_addons(&settings).collect::<Vec<_>>())
        })
        .await?
        .into_iter()
        .flatten()
        .collect();

        log::debug!("Active addons for pack '{}': {:#?}", pack_name, addons);

        Ok(addons)
    }

    async fn get_launch_addons(
        &self,
        pack_name: &str,
        include_optionals: bool,
    ) -> anyhow::Result<Vec<PackAddon>> {
        let mut addons = self.get_active_addons(pack_name, include_optionals).await?;

        let config = self.get_pack(pack_name).await?;
        addons.extend(needed_cdlc_compat(&config)?);

        Ok(addons)
    }
}

/// The compat addon this machine needs for the pack's CDLC: only when the pack
/// enables a CDLC that is not installed here.
pub(crate) fn needed_cdlc_compat(config: &PackConfig) -> anyhow::Result<Option<PackAddon>> {
    let Some(cdlc) = config.enabled_cdlc else {
        return Ok(None);
    };

    if has_cdlc_installed(&cdlc).context("Failed to check if CDLC is installed")? {
        return Ok(None);
    }

    log::info!(
        "CDLC '{:?}' is not installed, its compat is needed instead",
        cdlc
    );

    Ok(config.cdlc_compat_addon())
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
