use clap::{Args, ValueEnum};
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use std::env::current_dir;

#[derive(Debug, Args)]
pub struct LaunchArgs {
    #[arg()]
    pub name: String,

    #[arg(long, value_enum, default_value_t = LaunchType::Steam)]
    pub launch_type: LaunchType,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum LaunchType {
    Steam,
    File,
}

pub fn launch_command(args: LaunchArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&current_dir()?)?;

    #[allow(unreachable_patterns)]
    match args.launch_type {
        LaunchType::Steam => handle.launch_via_steam(&args.name),
        #[cfg(target_os = "windows")]
        LaunchType::File => handle.launch_via_executable(&args.name),
        _ => handle.launch_via_steam(&args.name),
    }
}
