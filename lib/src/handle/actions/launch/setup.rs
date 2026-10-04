use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::handle::externals::load_externals::LoadExternals;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::util::dirs::arma_install::find_arma_install;
use crate::util::dirs::linux_setup::LinuxLaunchSetup;
use crate::util::dirs::linux_setup::compute_linux_launch_setup;
use anyhow::{Context, anyhow};
use std::path::Path;
use std::path::PathBuf;

impl ClientRepoHandle {
    /// Describes the one-time Steam setup this repo needs on Linux: the string
    /// to paste into Arma's launch options, and the `flatpak override` command
    /// when mods sit outside the sandbox.
    pub fn linux_launch_setup(&self) -> anyhow::Result<LinuxLaunchSetup> {
        let arma = find_arma_install().context("Failed to find the Arma 3 installation")?;

        Ok(compute_linux_launch_setup(
            &arma.steam,
            &arma.install_dir,
            &arma.libraries,
            &self.mod_roots()?,
        ))
    }

    /// The directories this repo's mods live in: the repo itself, which holds
    /// every pack's addons, plus the directory each external addon sits in.
    fn mod_roots(&self) -> anyhow::Result<Vec<PathBuf>> {
        let repo_path = self
            .get_repo_path()
            .canonicalize()
            .context("Failed to resolve the repo path")?;

        let mut roots = vec![repo_path];

        for pack_name in self.all_pack_names() {
            let externals = self.load_externals(&pack_name).context(anyhow!(
                "Failed to load external addons for pack {pack_name:?}"
            ))?;

            for external in externals.iter().filter(|e| e.enabled) {
                let path = PathBuf::from(&external.path)
                    .canonicalize()
                    .context(format!(
                        "Failed to resolve the path of external addon {external:?}"
                    ))?;

                // The directory the addon sits in, not the addon itself, so
                // that a sibling added later is already covered.
                roots.push(path.parent().map(Path::to_path_buf).unwrap_or(path))
            }
        }

        Ok(roots)
    }

    /// Every pack in the repo, remote and local alike.
    fn all_pack_names(&self) -> Vec<String> {
        use crate::handle::reading::get_repo_info::GetRepoInfo;

        self.get_config()
            .packs
            .iter()
            .chain(self.user_settings().local_packs.iter())
            .cloned()
            .collect()
    }
}
