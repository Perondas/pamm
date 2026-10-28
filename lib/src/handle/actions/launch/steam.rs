use crate::handle::actions::launch::launch_pack::LaunchOutcome;
use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::handle::reading::get_pack::GetPack;
use crate::models::cdlc::Cdlc;
use anyhow::Context;
use log::{debug, info};

impl ClientRepoHandle {
    pub(super) async fn launch_via_steam(
        &self,
        pack_name: &str,
        addon_paths: &[String],
        cdlc: Option<Cdlc>,
    ) -> anyhow::Result<LaunchOutcome> {
        info!("Launching pack '{}' via Steam", pack_name);

        let mut launch_url = String::from("steam://rungameid/107410// -nolauncher ");

        let (pack_config, settings) = self.get_pack_with_settings(pack_name).await?;

        for param in pack_config.client_params {
            launch_url.push_str(&param);
            launch_url.push(' ');
        }

        for param in settings.launch_params {
            launch_url.push_str(&param);
            launch_url.push(' ');
        }

        // CDLCs are addressed by their short form, not by a path
        let mods = addon_paths
            .iter()
            .map(String::as_str)
            .chain(cdlc.as_ref().map(Cdlc::short_form))
            .collect::<Vec<_>>();

        let addons_combined = format!("\"-mod={}\"", mods.join(";"));

        launch_url.push_str(&urlencoding::encode(&addons_combined));

        debug!("Steam launch URL: {}", launch_url);
        open::that(launch_url).context("Failed to launch pack via Steam")?;

        // No preset file on this platform: the mod list rides in the URL.
        Ok(LaunchOutcome::default())
    }
}
