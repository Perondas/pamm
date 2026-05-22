#[cfg(not(target_os = "linux"))]
use crate::handle::client::reading::get_canonical_addon_paths::GetAddonPaths;
#[cfg(target_os = "linux")]
use crate::handle::client::reading::get_linux_addon_paths::GetLinuxAddonPaths;
use crate::handle::client::reading::get_pack::GetPack;
use crate::handle::client::client_repo_handle::ClientRepoHandle;
use anyhow::Context;
use log::{debug, info};

impl ClientRepoHandle {
    pub fn launch_via_steam(&self, pack_name: &str) -> anyhow::Result<()> {
        info!("Launching pack '{}' via Steam", pack_name);

        // On linux we need to have the load path be in the Arma directory.
        let addon_paths = cfg_select! {
            target_os = "linux" => self.get_linux_addon_paths(pack_name),
            _ => self.get_canonical_addon_paths(pack_name)
        }?;

        debug!(
            "Resolved {} addon path(s) for pack '{}'",
            addon_paths.len(),
            pack_name
        );

        let mut launch_url = String::from("steam://rungameid/107410// -nolauncher ");

        let (pack_config, settings) = self.get_pack_with_settings(pack_name)?;

        for param in pack_config.client_params {
            launch_url.push_str(&param);
            launch_url.push(' ');
        }

        for param in settings.launch_params {
            launch_url.push_str(&param);
            launch_url.push(' ');
        }

        let addons_combined = format!("\"-mod={}\"", addon_paths.join(";"));

        launch_url.push_str(&urlencoding::encode(&addons_combined));

        debug!("Steam launch URL: {}", launch_url);
        open::that(launch_url).context("Failed to launch pack via Steam")
    }
}
