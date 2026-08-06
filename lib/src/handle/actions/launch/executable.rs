use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::handle::reading::get_canonical_addon_paths::GetAddonPaths;
use crate::handle::reading::get_pack::GetPack;
use crate::util::arma::find_arma_install_dir::find_arma_install_dir;
use anyhow::Context;
use log::{debug, info};
use std::os::windows::process::CommandExt;
use std::process::Command;

impl ClientRepoHandle {
    pub fn launch_via_executable(&self, pack_name: &str) -> anyhow::Result<()> {
        info!("Launching pack '{}' via executable", pack_name);

        let executable = find_arma_install_dir()
            .context("Failed to find Arma install directory")?
            .join("arma3_x64.exe");

        let addon_paths = self.get_canonical_addon_paths(pack_name)?;

        debug!(
            "Resolved {} addon path(s) for pack '{}'",
            addon_paths.len(),
            pack_name
        );

        let mut command = Command::new(executable);

        // Start arma as a new process group
        command.creation_flags(0x00000200);

        command.arg("-nolauncher");

        let (pack_config, settings) = self.get_pack_with_settings(pack_name)?;

        for param in pack_config.client_params {
            command.arg(param);
        }

        for param in settings.launch_params {
            command.arg(param);
        }

        let addons_combined = format!("-mod={}", addon_paths.join(";"));

        command.arg(addons_combined);

        debug!("Executable command: {:#?}", command);
        let _ = command
            .spawn()
            .context("Failed to launch pack via executable")?;

        Ok(())
    }
}
