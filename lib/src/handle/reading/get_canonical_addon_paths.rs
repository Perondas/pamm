use crate::handle::addons::ResolveAddons;
use crate::handle::externals::get_external_addons_paths::GetExternalAddonsPaths;
use crate::handle::optionals::get_optionals_paths::GetOptionalsPaths;
use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::io::fs::util::clean_path::canonicalize_and_clean_path;
use anyhow::{Context, anyhow};

pub trait GetAddonPaths {
    /// Gets the paths to all enabled addons of the pack (required, optional and
    /// external) as absolute, canonicalized path strings.
    fn get_canonical_addon_paths(
        &self,
        pack_name: &str,
        disable_optionals: bool,
    ) -> anyhow::Result<Vec<String>>;
}

impl<T> GetAddonPaths for T
where
    T: GetPack + GetRepoInfo,
{
    fn get_canonical_addon_paths(
        &self,
        pack_name: &str,
        disable_optionals: bool,
    ) -> anyhow::Result<Vec<String>> {
        log::debug!("Resolving canonical addon paths for pack '{}'", pack_name);

        let repo_path = self.get_repo_path();

        let required_addons = self.resolve_addons(pack_name)?;
        let optional_addons = if !disable_optionals {
            self.get_optional_paths(pack_name)?
        } else {
            vec![]
        };

        let addons = required_addons
            .iter()
            .chain(&optional_addons)
            .map(|p| repo_path.join(p))
            .map(canonicalize_and_clean_path)
            .collect::<anyhow::Result<Vec<_>>>()?;

        let externals = self
            .get_external_addon_paths(pack_name)
            .context(anyhow!("Failed to read external addons"))?;

        Ok([addons, externals].concat())
    }
}
