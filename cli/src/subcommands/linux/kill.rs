use crate::subcommands::linux::proton::detect_proton;
use clap::Args;

/// Stop everything running in Arma's Proton prefix, including the game
#[derive(Debug, Args)]
pub struct KillArgs {}

pub fn kill_command(_args: KillArgs) -> anyhow::Result<()> {
    detect_proton()?.kill_all()
}
