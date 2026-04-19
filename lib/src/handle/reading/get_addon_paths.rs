use crate::handle::addons::ResolveAddons;
use crate::handle::externals::get_external_addons_paths::GetExternalAddonsPaths;
use crate::handle::optionals::get_optionals_paths::GetOptionalsPaths;
use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::io::fs::util::clean_path::clean_path;
use anyhow::{Context, anyhow};

pub trait GetAddonPaths {
    fn get_addon_paths(&self, pack_name: &str) -> anyhow::Result<Vec<String>>;
}

impl<T> GetAddonPaths for T
where
    T: GetPack + GetRepoInfo,
{
    fn get_addon_paths(&self, pack_name: &str) -> anyhow::Result<Vec<String>> {
        log::debug!("Resolving addon paths for pack '{}'", pack_name);

        let addons = self
            .resolve_addons(pack_name)?
            .iter()
            .chain(&self.get_optional_paths(pack_name)?)
            .map(|p| {
                p.canonicalize()
                    .with_context(|| format!("Failed to canonicalize {:#?}", p))
            })
            .map(|p| p.map(clean_path))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let externals = self
            .get_external_addon_paths(pack_name)
            .context(anyhow!("Failed to read external addons"))?;

        Ok([addons, externals].concat())
    }
}
