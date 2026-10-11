use clap::Args;
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
