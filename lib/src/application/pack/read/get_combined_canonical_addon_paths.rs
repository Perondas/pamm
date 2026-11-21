use crate::application::addons::active::GetActiveAddons;
use crate::application::addons::externals::GetExternalsPaths;
use crate::application::pack::read::get_pack::GetPack;
use crate::application::pack::read::get_repo_info::GetRepoInfo;
use crate::infrastructure::fs::files::util::clean_path::canonicalize_and_clean_path;
use anyhow::{Context, anyhow};
use std::collections::HashSet;

pub trait GetAddonPaths {
    /// Gets the paths to all enabled addons of the pack (required, optional,
    /// workshop and external) as absolute, canonicalized path strings, including
    /// the CDLC compat when the pack's CDLC is not installed. Fails if any of
    /// these addons cannot be found.
    fn get_combined_canonical_addon_paths(
        &self,
        pack_name: &str,
        disable_optionals: bool,
    ) -> impl Future<Output = anyhow::Result<HashSet<String>>> + Send;
}

impl<T> GetAddonPaths for T
where
    T: GetPack + GetRepoInfo + Sync,
{
    async fn get_combined_canonical_addon_paths(
        &self,
        pack_name: &str,
        disable_optionals: bool,
    ) -> anyhow::Result<HashSet<String>> {
        log::debug!("Resolving canonical addon paths for pack '{}'", pack_name);

        let repo_path = self.get_repo_path();

        let mut addons = self
            .get_launch_addons(pack_name, !disable_optionals)
            .await?
            .iter()
            .map(|addon| {
                let path = addon.locate(repo_path)?.with_context(|| {
                    format!(
                        "Could not find addon {addon}. Make sure the pack is synced and \
                         its workshop addons are subscribed to"
                    )
                })?;
                canonicalize_and_clean_path(path)
            })
            .collect::<anyhow::Result<HashSet<_>>>()?;

        addons.extend(
            self.get_externals_paths(pack_name)
                .await
                .context(anyhow!("Failed to read external addons"))?,
        );

        Ok(addons)
    }
}
