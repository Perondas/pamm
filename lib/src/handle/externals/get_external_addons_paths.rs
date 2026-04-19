use crate::handle::reading::get_pack::GetPack;
use crate::io::fs::util::clean_path::clean_path;
use anyhow::Context;
use std::path::PathBuf;

impl<T> GetExternalAddonsPaths for T
where
    T: GetPack,
{
    fn get_external_addon_paths(&self, pack_name: &str) -> anyhow::Result<Vec<String>> {
        let (_, settings) = self.get_pack_with_settings(pack_name)?;

        settings
            .external_addons
            .iter()
            .filter(|addon| addon.enabled)
            .map(|addon| PathBuf::from(addon.path.to_owned()))
            .map(|p| {
                p.canonicalize()
                    .with_context(|| format!("Failed to canonicalize {:#?}", p))
            })
            .map(|p| p.map(clean_path))
            .collect::<anyhow::Result<Vec<_>>>()
    }
}

pub(in crate::handle) trait GetExternalAddonsPaths {
    fn get_external_addon_paths(&self, pack_name: &str) -> anyhow::Result<Vec<String>>;
}
