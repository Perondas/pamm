use crate::handle::optionals::optional_addon::OptionalAddon;
use crate::handle::repo_handle::RepoHandle;

impl RepoHandle {
    pub fn save_optionals(
        &self,
        pack_name: &str,
        optionals: &[OptionalAddon],
    ) -> anyhow::Result<()> {
        let (_, mut settings) = self.get_pack_with_settings(pack_name)?;

        let enabled = optionals
            .iter()
            .filter(|optional| optional.enabled)
            .map(|optional| optional.name.to_owned())
            .collect();

        settings.enabled_optionals = enabled;

        self.write_named(&settings, pack_name)?;

        Ok(())
    }
}
