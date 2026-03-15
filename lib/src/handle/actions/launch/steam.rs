use crate::handle::repo_handle::RepoHandle;
use log::{debug, info};

impl RepoHandle {
    pub fn launch_via_steam(&self, pack_name: &str) -> anyhow::Result<()> {
        info!("Launching pack '{}' via Steam", pack_name);

        let addons = self.get_addon_paths(pack_name)?;
        
        debug!(
            "Resolved {} addon path(s) for pack '{}'",
            addons.len(),
            pack_name
        );

        let mut launch_url = String::from("steam://rungameid/107410// -nolauncher ");

        let pack_config = self.get_pack(pack_name)?;

        for param in pack_config.client_params {
            launch_url.push_str(&param);
            launch_url.push(' ');
        }

        let addons_combined = format!("\"-mod={}\"", addons.join(";"));

        launch_url.push_str(&format!("{}", urlencoding::encode(&addons_combined)));

        debug!("Steam launch URL: {}", launch_url);
        open::that(launch_url)?;

        Ok(())
    }
}
