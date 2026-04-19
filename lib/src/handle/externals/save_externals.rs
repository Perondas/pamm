use crate::handle::externals::external_addon::ExternalAddon;
use crate::handle::reading::get_pack::GetPack;
use crate::handle::repo_handle::RepoHandle;
use std::collections::HashSet;

impl RepoHandle {
    pub fn save_externals(
        &self,
        pack_name: &str,
        externals: &[ExternalAddon],
    ) -> anyhow::Result<()> {
        let (_, mut settings) = self.get_pack_with_settings(pack_name)?;

        settings.external_addons = externals
            .iter()
            .map(|e| e.to_owned())
            .map(|mut e| {
                e.resolve_name()?;
                Ok(e)
            })
            .collect::<anyhow::Result<HashSet<_>>>()?;

        self.write_named(&settings, pack_name)?;

        Ok(())
    }
}
