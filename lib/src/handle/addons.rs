use crate::handle::reading::get_pack::GetPack;
use crate::io::files::name_consts::ADDONS_DIR_NAME;
use std::path::PathBuf;

impl<T> ResolveAddons for T
where
    T: GetPack + Sync,
{
    /// Returns relative paths of the form `<pack>/addons/<addon>`, relative to
    /// the repo root (join with the repo path to get usable filesystem paths).
    async fn resolve_addons(&self, pack_name: &str) -> anyhow::Result<Vec<PathBuf>> {
        let mut addons = Vec::new();
        let mut current = Some(pack_name.to_string());

        // Own addons first, then each ancestor's in turn.
        while let Some(pack_name) = current {
            let pack_config = self.get_pack(&pack_name).await?;

            let addon_dir = PathBuf::from(&pack_name).join(ADDONS_DIR_NAME);

            addons.extend(
                pack_config
                    .addons
                    .iter()
                    .filter(|(_, addon_settings)| !addon_settings.is_optional)
                    .map(|addon| addon_dir.join(addon.0)),
            );

            current = pack_config.parent;
        }

        Ok(addons)
    }
}

pub(in crate::handle) trait ResolveAddons {
    /// Gets the paths to the pack's non-optional addons (including inherited ones).
    /// The returned paths are relative to the repo root, e.g. `<pack>/addons/<addon>`.
    fn resolve_addons(
        &self,
        pack_name: &str,
    ) -> impl Future<Output = anyhow::Result<Vec<PathBuf>>> + Send;
}
