use crate::handle::reading::get_pack::GetPack;
use crate::io::files::name_consts::ADDONS_DIR_NAME;
use std::path::PathBuf;

impl<T> ResolveAddons for T
where
    T: GetPack,
{
    /// Returns relative paths of the form `<pack>/addons/<addon>`, relative to
    /// the repo root (join with the repo path to get usable filesystem paths).
    fn resolve_addons(&self, pack_name: &str) -> anyhow::Result<Vec<PathBuf>> {
        let pack_config = self.get_pack(pack_name)?;

        let addon_dir = PathBuf::from(pack_name).join(ADDONS_DIR_NAME);

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
    /// Gets the paths to the pack's non-optional addons (including inherited ones).
    /// The returned paths are relative to the repo root, e.g. `<pack>/addons/<addon>`.
    fn resolve_addons(&self, pack_name: &str) -> anyhow::Result<Vec<PathBuf>>;
}
