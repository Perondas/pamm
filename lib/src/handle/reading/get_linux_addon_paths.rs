use crate::handle::addons::ResolveAddons;
use crate::handle::externals::get_external_addons_paths::GetExternalAddonsPaths;
use crate::handle::optionals::GetOptionalsPaths;
use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::io::fs::util::symlink::create_or_recreate_symlink;
use crate::util::linux::get_arma_install_dir::get_arma_install_dir;
use anyhow::{anyhow, Context};
use std::fs::create_dir_all;
use std::path::Path;

pub trait GetLinuxAddonPaths {
    /// Gets the paths to all enabled addons of the pack as relative path strings
    /// of the form `pamm/<repo name>/<addon>` or `pamm/externals/<addon>`,
    /// relative to the Arma install directory (where the `pamm` symlinks are
    /// created), suitable for use as `-mod=` launch parameters.
    fn get_linux_addon_paths(&self, pack_name: &str) -> anyhow::Result<Vec<String>>;
}

impl<T> GetLinuxAddonPaths for T
where
    T: GetPack + GetRepoInfo + GetExternalAddonsPaths,
{
    fn get_linux_addon_paths(&self, pack_name: &str) -> anyhow::Result<Vec<String>> {
        log::debug!("Resolving addon paths for pack '{}'", pack_name);

        let arma_install_dir =
            get_arma_install_dir().context(anyhow!("Failed to get Arma installation directory"))?;
        log::debug!("Found Arma installation directory: {:?}", arma_install_dir);

        let pamm_dir = arma_install_dir.join("pamm");
        let externals_dir = pamm_dir.join("externals");

        log::trace!("Ensuring pamm directories exist at {:?} and {:?}", pamm_dir, externals_dir);
        create_dir_all(&pamm_dir)
            .context(anyhow!("Failed to create PAMM directory at {:?}", pamm_dir))?;
        create_dir_all(&externals_dir).context(anyhow!(
            "Failed to create externals directory at {:?}",
            externals_dir
        ))?;

        let symlink_path = pamm_dir.join(&self.get_config().name);
        log::debug!("Creating repo symlink for pack: {:?}", symlink_path);

        create_or_recreate_symlink(self.get_repo_path(), &symlink_path)?;

        let mut addons = self
            .resolve_addons(pack_name)?
            .iter()
            .chain(&self.get_optional_paths(pack_name)?)
            .map(|p| {
                p.to_str()
                    .map(|s| s.to_string())
                    .context(anyhow!("Failed to convert path to string: {:?}", p))
            })
            .collect::<anyhow::Result<Vec<_>>>()?
            .into_iter()
            .map(|rel| format!("pamm/{}/{}", self.get_config().name, rel))
            .collect::<Vec<_>>();

        let externals: Vec<String> = self
            .get_external_addon_paths(pack_name)
            .context(anyhow!("Failed to read external addons"))?;

        log::debug!("Found {} external addons", externals.len());

        for external in externals {
            let external_path = Path::new(&external);
            if let Some(folder_name) = external_path.file_name() {
                let link_path = externals_dir.join(folder_name);
                log::trace!("Processing external addon {:?} -> {:?}", external_path, link_path);
                create_or_recreate_symlink(external_path, &link_path)?;
                if let Some(folder_name_str) = folder_name.to_str() {
                    addons.push(format!("pamm/externals/{}", folder_name_str));
                }
            } else {
                log::warn!("Failed to determine folder name for external addon {:?}", external_path);
            }
        }

        log::debug!("Successfully resolved {} total addon paths", addons.len());

        Ok(addons)
    }
}
