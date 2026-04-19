use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::io::name_consts::get_pack_addon_directory_name;
use std::path::PathBuf;

impl<T> ResolveAddons for T
where
    T: GetPack + GetRepoInfo,
{
    fn resolve_addons(&self, pack_name: &str) -> anyhow::Result<Vec<PathBuf>> {
        let pack_config = self.get_pack(pack_name)?;

        let addon_dir = self
            .get_repo_path()
            .join(get_pack_addon_directory_name(&pack_config.name));

        let own_addons = pack_config
            .addons
            .iter()
            .filter(|(_, addon_settings)| !addon_settings.is_optional)
            .map(|addon| addon_dir.join(addon.0));

        let inherited = if let Some(parent) = pack_config.parent {
            self.resolve_addons(&parent)?
        } else {
            vec![]
        };

        Ok(own_addons.chain(inherited).collect())
    }
}

pub(in crate::handle) trait ResolveAddons {
    fn resolve_addons(&self, pack_name: &str) -> anyhow::Result<Vec<PathBuf>>;
}
