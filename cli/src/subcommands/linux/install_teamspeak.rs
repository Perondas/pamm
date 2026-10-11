use crate::subcommands::linux::proton::detect_proton;
use anyhow::{anyhow, bail};
use clap::Args;
use pamm_lib::util::proton::teamspeak::{find_teamspeak, install_teamspeak};
use std::path::PathBuf;

/// Install TeamSpeak into Arma's Proton prefix
#[derive(Debug, Args)]
pub struct InstallTeamspeakArgs {
    /// Path to the TeamSpeak 3 installer
    #[arg()]
    pub installer: PathBuf,

    /// Show the installer wizard. Detaults to false
    #[arg(long, default_value = "false")]
    pub interactive: bool,
}

pub fn install_teamspeak_command(args: InstallTeamspeakArgs) -> anyhow::Result<()> {
    if args
        .installer
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        != "exe"
    {
        bail!(anyhow!(
            "It looks like you are not using the windows installer as the file does not end in .exe"
        ));
    }

    let proton = detect_proton()?;

    println!("About to install teamspeak. This may mess up your terminal output");

    install_teamspeak(&proton, &args.installer, !args.interactive)?;

    match find_teamspeak(&proton.prefix()) {
        Some(exe) => println!("TeamSpeak installed at {}", exe.display()),
        None => println!("The installer finished, but TeamSpeak could not be found in the prefix."),
    }

    Ok(())
}
