use clap::{Args, ValueEnum};
use pamm_lib::handle::actions::launch::launch_pack::LaunchParams;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use std::env::current_dir;

#[derive(Debug, Args)]
pub struct LaunchArgs {
    #[arg()]
    pub name: String,

    #[arg(long, value_enum, default_value_t = LaunchMode::Steam)]
    pub launch_type: LaunchMode,

    #[arg(long)]
    pub no_optionals: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[non_exhaustive]
pub enum LaunchMode {
    Steam,
    #[cfg(target_os = "windows")]
    File,
}

impl From<LaunchMode> for pamm_lib::handle::actions::launch::launch_pack::LaunchMode {
    fn from(mode: LaunchMode) -> Self {
        match mode {
            #[cfg(target_os = "windows")]
            LaunchMode::File => Self::Executable,
            _ => Self::Steam,
        }
    }
}

pub fn launch_command(args: LaunchArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&current_dir()?)?;
    let launch_params = LaunchParams::new(args.launch_type.into(), args.no_optionals);
    let outcome = handle.launch_pack(&args.name, &launch_params)?;

    if let Some(preset_path) = outcome.preset_path {
        println!("Wrote preset to {preset_path}");
        println!("If Arma starts without mods, check `pamm setup`.");
    }

    Ok(())
}
