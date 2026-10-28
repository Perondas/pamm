use crate::handle::actions::launch::launch_pack::LaunchOutcome;
use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::handle::reading::get_pack::GetPack;
use crate::models::cdlc::Cdlc;
use crate::util::dirs::find_steam_dir::find_steam_dir;
use anyhow::Context;
use log::{debug, info};
use std::os::windows::process::CommandExt;
use std::process::Command;

impl ClientRepoHandle {
    pub(super) async fn launch_via_executable(
        &self,
        pack_name: &str,
        addon_paths: &[String],
        cdlc: Option<Cdlc>,
    ) -> anyhow::Result<LaunchOutcome> {
        info!("Launching pack '{}' via executable", pack_name);

        let steam_executable = find_steam_dir()
            .context("Failed to find Arma install directory")?
            .join("steam.exe");

        let (pack_config, settings) = self.get_pack_with_settings(pack_name).await?;

        let mut command = Command::new(steam_executable);

        // Start arma as a new process group

        command.creation_flags(0x00000200);

        command.arg("-applaunch");
        command.arg("107410"); // Arma 3 Steam App ID
        command.arg("-nolauncher");

        for param in pack_config.client_params {
            command.arg(param);
        }

        for param in settings.launch_params {
            command.arg(param);
        }

        // CDLCs are addressed by their short form, not by a path
        let mods = addon_paths
            .iter()
            .map(String::as_str)
            .chain(cdlc.as_ref().map(Cdlc::short_form))
            .collect::<Vec<_>>();

        let addons_combined = format!("-mod={}", mods.join(";"));

        command.arg(addons_combined);

        debug!("Executable command: {:#?}", command);
        let _ = command
            .spawn()
            .context("Failed to launch pack via executable")?;

        // No preset file on this platform: the mod list rides in the arguments.
        Ok(LaunchOutcome::default())
    }
}
