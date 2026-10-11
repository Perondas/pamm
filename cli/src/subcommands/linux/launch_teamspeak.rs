use crate::subcommands::linux::proton::detect_proton;
use anyhow::Context;
use clap::Args;
use pamm_lib::util::proton::teamspeak::{find_teamspeak, launch_teamspeak};

/// Launch TeamSpeak inside Arma's Proton prefix
#[derive(Debug, Args)]
pub struct LaunchTeamspeakArgs {}

pub fn launch_teamspeak_command(args: LaunchTeamspeakArgs) -> anyhow::Result<()> {
    let proton = detect_proton()?;

    let exe = find_teamspeak(&proton.prefix())
        .context("TeamSpeak is not installed. Run `linux install-teamspeak` first.")?;

    launch_teamspeak(&proton, &exe, None)?;

    Ok(())
}
