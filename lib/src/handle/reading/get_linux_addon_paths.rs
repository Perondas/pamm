use crate::handle::addons::ResolveAddons;
use crate::handle::optionals::GetOptionalsPaths;
use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::util::linux::get_arma_install_dir::get_arma_install_dir;
use anyhow::{anyhow, Context};
use std::fs::{create_dir_all, symlink_metadata};
use std::os::unix::fs::symlink;

pub trait GetLinuxAddonPaths {
    fn get_linux_addon_paths(&self, pack_name: &str) -> anyhow::Result<Vec<String>>;
}

impl<T> GetLinuxAddonPaths for T
where
    T: GetPack + GetRepoInfo,
{
    fn get_linux_addon_paths(&self, pack_name: &str) -> anyhow::Result<Vec<String>> {
        log::debug!("Resolving addon paths for pack '{}'", pack_name);

        let arma_install_dir =
            get_arma_install_dir().context(anyhow!("Failed to get Arma installation directory"))?;

        let pamm_dir = arma_install_dir.join("pamm");

        create_dir_all(&pamm_dir)
            .context(anyhow!("Failed to create PAMM directory at {:?}", pamm_dir))?;

        let symlink_path = pamm_dir.join(&self.get_config().name);

        if symlink_path.exists() {
            if symlink_metadata(&symlink_path)?.file_type().is_symlink() {
                std::fs::remove_file(&symlink_path).context(anyhow!(
                    "Failed to remove existing symlink at {:?}",
                    symlink_path
                ))?;
            } else {
                return Err(anyhow!(
                    "Path {:?} already exists and is not a symlink",
                    symlink_path
                ));
            }
        }

        symlink(self.get_repo_path(), symlink_path)?;

        let addons = self
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

        /*        let externals = self
        .get_external_addon_paths(pack_name)
        .context(anyhow!("Failed to read external addons"))?;*/

        Ok([addons /*externals*/].concat())
    }
}
