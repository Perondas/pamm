use crate::handle::client_repo_handle::ClientRepoHandle;
#[cfg(target_os = "linux")]
use std::path::Path;

pub use crate::util::dirs::linux_setup::LinuxLaunchSetup;
pub use crate::util::dirs::steam_install::SteamFlavour;

impl ClientRepoHandle {
    /// Describes the one-time Steam setup this repo needs on Linux: the string
    /// to paste into Arma's launch options, and the `flatpak override` command
    /// when mods sit outside the sandbox.
    ///
    /// Per repo rather than per pack. What Steam needs access to is the
    /// directories mods are kept in, and every pack in a repo shares them, so
    /// asking pack by pack would produce the same answer over and over and
    /// invite the user to paste a string that only covers one of them.
    ///
    /// Detection only — nothing here writes Steam's configuration or runs
    /// `flatpak`.
    ///
    /// The signature is the same on every platform so that the flutter_rust_bridge
    /// codegen, which does not evaluate `cfg`, produces identical Dart wherever
    /// it runs. Only the body is platform-specific.
    #[cfg(target_os = "linux")]
    pub fn linux_launch_setup(&self) -> anyhow::Result<LinuxLaunchSetup> {
        use crate::util::dirs::arma_install::find_arma_install;
        use crate::util::dirs::linux_setup::linux_launch_setup;
        use anyhow::Context;

        let arma = find_arma_install().context("Failed to find the Arma 3 installation")?;

        Ok(linux_launch_setup(
            &arma.steam,
            &arma.install_dir,
            &arma.libraries,
            &self.mod_roots()?,
        ))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn linux_launch_setup(&self) -> anyhow::Result<LinuxLaunchSetup> {
        anyhow::bail!("Steam launch setup is only needed on Linux")
    }

    /// The directories this repo's mods live in: the repo itself, which holds
    /// every pack's addons, plus the directory each external addon sits in.
    ///
    /// Unresolvable externals are skipped rather than failing the whole thing —
    /// setup has to be available precisely when something is not yet in place.
    #[cfg(target_os = "linux")]
    fn mod_roots(&self) -> anyhow::Result<Vec<std::path::PathBuf>> {
        use crate::handle::externals::load_externals::LoadExternals;
        use crate::handle::reading::get_repo_info::GetRepoInfo;
        use anyhow::Context;
        use std::path::PathBuf;

        let repo_path = self
            .get_repo_path()
            .canonicalize()
            .context("Failed to resolve the repo path")?;

        let mut roots = vec![repo_path];

        for pack_name in self.all_pack_names() {
            let externals = match self.load_externals(&pack_name) {
                Ok(externals) => externals,
                Err(e) => {
                    log::warn!("Skipping externals of pack '{pack_name}': {e:#}");
                    continue;
                }
            };

            for external in externals.iter().filter(|external| external.enabled) {
                let path = PathBuf::from(&external.path);
                match path.canonicalize() {
                    // The directory the addon sits in, not the addon itself, so
                    // that a sibling added later is already covered.
                    Ok(resolved) => {
                        roots.push(resolved.parent().map(Path::to_path_buf).unwrap_or(resolved))
                    }
                    Err(e) => log::warn!("Skipping external addon {path:?}: {e}"),
                }
            }
        }

        Ok(roots)
    }

    /// Every pack in the repo, remote and local alike.
    #[cfg(target_os = "linux")]
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
