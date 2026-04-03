use crate::handle::optionals::optional_addon::OptionalAddon;
use crate::handle::repo_handle::RepoHandle;

impl RepoHandle {
    pub fn save_optionals(
        &self,
        pack_name: &str,
        optionals: &[OptionalAddon],
    ) -> anyhow::Result<()> {
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

        self.write_named(&settings, pack_name)?;
        
        if let Some(parent) = &config.parent {
            self.save_optionals(parent, optionals)?;
        }

        Ok(())
    }
}
