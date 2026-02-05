use clap::Args;
use pamm_lib::commands::steam_launch::launch_via_steam;
use std::env::current_dir;

#[derive(Debug, Args)]
pub struct LaunchArgs {
    #[arg()]
    pub name: String,
}

pub fn launch_command(args: LaunchArgs) -> anyhow::Result<()> {
    let current_dir = current_dir()?;

    launch_via_steam(&current_dir, &args.name)
}
