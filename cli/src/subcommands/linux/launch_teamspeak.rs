use crate::subcommands::linux::proton::detect_proton;
use anyhow::Context;
use clap::Args;
use pamm_lib::util::proton::teamspeak::{find_teamspeak, launch_teamspeak};

/// Launch TeamSpeak inside Arma's Proton prefix
#[derive(Debug, Args)]
pub struct LaunchTeamspeakArgs {
    /// A ts3server:// URL to connect to
    #[arg()]
    pub server_url: Option<String>,
}

pub fn launch_teamspeak_command(args: LaunchTeamspeakArgs) -> anyhow::Result<()> {
    let proton = detect_proton()?;

    let exe = find_teamspeak(&proton.prefix())
        .context("TeamSpeak is not installed. Run `linux install-teamspeak` first.")?;

    launch_teamspeak(&proton, &exe, args.server_url.as_deref())?;

    Ok(())
}
