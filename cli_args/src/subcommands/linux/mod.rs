use clap::Subcommand;
use explorer::ExplorerArgs;
use install_teamspeak::InstallTeamspeakArgs;
use kill::KillArgs;
use launch_teamspeak::LaunchTeamspeakArgs;

pub mod explorer;
pub mod install_teamspeak;
pub mod kill;
pub mod launch_teamspeak;

/// Show the one-time Steam setup this repo needs on Linux, or manage Arma's Proton prefix
#[derive(Debug, clap::Args)]
pub struct LinuxArgs {
    #[clap(subcommand)]
    pub command: Option<LinuxSubcommand>,
}

#[derive(Debug, Subcommand)]
pub enum LinuxSubcommand {
    InstallTeamspeak(InstallTeamspeakArgs),
    Teamspeak(LaunchTeamspeakArgs),
    Explorer(ExplorerArgs),
    Kill(KillArgs),
}
