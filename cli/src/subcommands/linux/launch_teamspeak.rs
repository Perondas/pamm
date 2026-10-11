use crate::subcommands::linux::proton::detect_proton;
use anyhow::Context;
use clap::Args;
use pamm_lib::util::proton::teamspeak::{TEAMSPEAK_DOWNLOAD_URL, find_teamspeak, launch_teamspeak};

/// Launch TeamSpeak inside Arma's Proton prefix
#[derive(Debug, Args)]
pub struct LaunchTeamspeakArgs {}

pub fn launch_teamspeak_command(_args: LaunchTeamspeakArgs) -> anyhow::Result<()> {
    let proton = detect_proton()?;

    let exe = find_teamspeak(&proton.prefix()).with_context(|| {
        format!(
            "TeamSpeak is not installed. Download the Windows version from \
             {TEAMSPEAK_DOWNLOAD_URL} and run `linux install-teamspeak <installer>`."
        )
    })?;

    launch_teamspeak(&proton, &exe, None)?;

    Ok(())
}
