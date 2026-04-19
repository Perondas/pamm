use crate::handle::externals::external_addon::ExternalAddon;
use crate::handle::reading::get_pack::GetPack;
use crate::handle::repo_handle::RepoHandle;

impl RepoHandle {
    pub fn load_externals(&self, pack_name: &str) -> anyhow::Result<Vec<ExternalAddon>> {
        let (_, settings) = self.get_pack_with_settings(pack_name)?;

        Ok(settings.external_addons.into_iter().collect())
    }
}
