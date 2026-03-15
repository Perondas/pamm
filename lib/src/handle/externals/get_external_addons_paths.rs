use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::util::clean_path::clean_path;
use anyhow::Context;
use std::path::PathBuf;

impl RepoHandle {
    pub(in crate::handle) fn get_external_addon_paths(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<Vec<String>> {
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
