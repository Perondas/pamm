use crate::handle::optionals::optional_addon::OptionalAddon;
use crate::handle::repo_handle::RepoHandle;

impl RepoHandle {
    pub fn save_optionals(
        &self,
        pack_name: &str,
        optionals: &[OptionalAddon],
    ) -> anyhow::Result<()> {
        let (pack_config, mut settings) = self.get_pack_with_settings(pack_name)?;

        let directly_enabled = optionals
            .iter()
            .filter(|optional| optional.enabled)
            .map(|optional| optional.name.to_owned());

        // Crude way of deactivating all optionals before re-enabling the ones that are enabled, but it works and is simple
        settings
            .enabled_optionals
            .iter_mut()
            .for_each(|(_, setting)| {
                setting.is_enabled = false;
            });

        // Filter out all totally disabled optionals
        settings.enabled_optionals = settings
            .enabled_optionals
            .into_iter()
            .filter(|(_, setting)| setting.is_enabled || setting.is_transitive_enabled)
            .collect();

        for enabled in directly_enabled {
            settings
                .enabled_optionals
                .entry(enabled)
                .or_default()
                .is_enabled = true;
        }

        self.write_named(&settings, pack_name)?;

        if let Some(parent) = &pack_config.parent {
            let dependants_transitives = settings
                .enabled_optionals
                .iter()
                .filter(|(_, setting)| setting.is_transitive_enabled)
                .map(|(name, _)| name);

            let own_transitives = settings
                .enabled_optionals
                .iter()
                .filter(|(_, setting)| setting.is_enabled)
                .map(|(name, _)| name)
                .filter(|name| pack_config.addons.get(*name).is_none());

            self.save_transitive_optionals(
                parent,
                dependants_transitives.chain(own_transitives).collect(),
            )?;
        }

        Ok(())
    }

    fn save_transitive_optionals(
        &self,
        pack_name: &str,
        optionals: Vec<&String>,
    ) -> anyhow::Result<()> {
        let (pack_config, mut settings) = self.get_pack_with_settings(pack_name)?;

        let own_optionals = optionals
            .iter()
            .filter(|name| pack_config.addons.get(**name).is_some())
            .cloned()
            .cloned();

        for optional in own_optionals {
            settings
                .enabled_optionals
                .entry(optional)
                .or_default()
                .is_transitive_enabled = true;
        }

        self.write_named(&settings, pack_name)?;

        if let Some(parent) = &pack_config.parent {
            self.save_transitive_optionals(parent, optionals)?;
        }

        // TODO: Check that no optionals are left over at the end

        Ok(())
    }
}
