use crate::handle::client_repo_handle::ClientRepoHandle;
#[cfg(target_os = "linux")]
use crate::handle::reading::get_linux_addon_paths::GetLinuxAddonPaths;
use crate::handle::reading::get_pack::GetPack;
use anyhow::Context;
use log::{debug, info};

impl ClientRepoHandle {
     pub(super) fn launch_via_steam(&self, pack_name: &str, addon_paths: &[String]) -> anyhow::Result<()> {
        info!("Launching pack '{}' via Steam", pack_name);

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
