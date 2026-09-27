use crate::handle::actions::launch::launch_pack::LaunchOutcome;
use crate::handle::actions::launch::preset::Preset;
use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::handle::reading::get_pack::GetPack;
use crate::util::dirs::arma_install::{ARMA_APP_ID, find_arma_install};
use crate::util::dirs::container_path::to_arma_path;
use anyhow::Context;
use log::{debug, info, warn};
use std::path::Path;

impl ClientRepoHandle {
    /// Launches through Steam by writing a preset file and opening a
    /// `steam://run` URL that points at it.
    pub(super) fn launch_via_steam(
        &self,
        pack_name: &str,
        addon_paths: &[String],
    ) -> anyhow::Result<LaunchOutcome> {
        info!("Launching pack '{}' via Steam", pack_name);

        let arma = find_arma_install().context("Failed to find the Arma 3 installation")?;

        // Host paths might not be what the game sees. Under Flatpak the persisted
        // home is mounted elsewhere, and a host path there would resolve
        // successfully to the wrong directory rather than failing loudly.
        let arma_mod_paths = addon_paths
            .iter()
            .map(|path| to_arma_path(&arma.steam.container_path(Path::new(path))))
            .collect::<anyhow::Result<Vec<_>>>()?;

        // Earlier versions symlinked mods into <arma>/pamm/. Warn the user
        let legacy_dir = arma.install_dir.join("pamm");
        if legacy_dir.exists() {
            warn!(
                "{legacy_dir:?} is left over from an older pamm and is no longer used. \
                 It is safe to delete."
            );
        }

        let (pack_config, settings) = self.get_pack_with_settings(pack_name)?;
        let params = [pack_config.client_params, settings.launch_params].concat();

        let preset = Preset::new(pack_name.to_string(), params, arma_mod_paths);

        let preset_path = preset.write_to(&arma.install_dir)?;

        info!("Wrote preset to {preset_path:?}");

        let launch_url = preset_launch_url(&preset_path);

        debug!("Steam launch URL: {launch_url}");
        open::that(launch_url).context("Failed to launch pack via Steam")?;

        Ok(LaunchOutcome {
            preset_path: Some(preset_path.display().to_string()),
        })
    }
}

/// The URL that launches Arma with a preset.
fn preset_launch_url(path_to_par: impl AsRef<Path>) -> String {
    let file_name = path_to_par
        .as_ref()
        .file_name()
        .expect("Somehow we have a file with no name?")
        .to_string_lossy();
    format!("steam://run/{ARMA_APP_ID}//-par={file_name}/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_the_preset_launch_url() {
        assert_eq!(
            preset_launch_url("pamm_core.txt"),
            "steam://run/107410//-par=pamm_core.txt/"
        );
    }
}
