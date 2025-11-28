use clap::Args;
use pamm_lib::pack::pack_config::PackConfig;

#[derive(Debug, Args)]
pub struct LaunchArgs {
    /*/// Force refresh all addons, ignoring cached state
    #[arg(short, long)]
    pub remote: Url,*/
}

pub fn launch_command(args: LaunchArgs) -> anyhow::Result<()> {
    let local_config = PackConfig::(&std::env::current_dir()?)?;
}
