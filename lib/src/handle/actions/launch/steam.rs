use crate::handle::reading::get_canonical_addon_paths::GetAddonPaths;
use crate::handle::reading::get_linux_addon_paths::GetLinuxAddonPaths;
use crate::handle::reading::get_pack::GetPack;
use crate::handle::repo_handle::RepoHandle;
use log::{debug, info};

impl RepoHandle {
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

        launch_url.push_str(&format!("{}", urlencoding::encode(&addons_combined)));

        debug!("Steam launch URL: {}", launch_url);
        open::that(launch_url)?;

        Ok(())
    }
}
