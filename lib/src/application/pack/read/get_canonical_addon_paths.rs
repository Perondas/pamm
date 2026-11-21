use crate::application::addons::externals::GetExternalsPaths;
use crate::application::addons::optionals::GetOptionalsPaths;
use crate::application::addons::resolve::ResolveAddons;
use crate::application::pack::read::get_pack::GetPack;
use crate::application::pack::read::get_repo_info::GetRepoInfo;
use crate::domain::cdlc::Cdlc;
use crate::infrastructure::fs::files::util::clean_path::canonicalize_and_clean_path;
use anyhow::{Context, anyhow};
use std::collections::HashSet;

pub trait GetAddonPaths {
    /// Gets the paths to all enabled addons of the pack (required, optional and
    /// external) as absolute, canonicalized path strings.
    fn get_combined_canonical_addon_paths(
        &self,
        pack_name: &str,
        disable_optionals: bool,
        cdlc: Option<Cdlc>,
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
        cdlc: Option<Cdlc>,
    ) -> anyhow::Result<HashSet<String>> {
        log::debug!("Resolving canonical addon paths for pack '{}'", pack_name);

        let repo_path = self.get_repo_path();

        let required_addons = self.resolve_addons(pack_name).await?;
        let optional_addons = if !disable_optionals {
            self.get_optionals_paths(pack_name).await?
        } else {
            vec![]
        };

        let mut addons = required_addons
            .iter()
            .chain(&optional_addons)
            .map(|p| repo_path.join(p))
            .map(canonicalize_and_clean_path)
            .collect::<anyhow::Result<HashSet<_>>>()?;

        let mut externals = self
            .get_externals_paths(pack_name)
            .await
            .context(anyhow!("Failed to read external addons"))?;

        if let Some(cldc) = cdlc {

        }

        addons.extend(externals);

        Ok(addons)
    }
}
