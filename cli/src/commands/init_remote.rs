use clap::Args;
use pamm_lib::repo::repo_config::RepoConfig;
use url::Url;

#[derive(Debug, Args)]
pub struct InitRemoteArgs {
    /// Force refresh all addons, ignoring cached state
    #[arg()]
    pub remote: Url,
}

pub fn init_remote_command(args: InitRemoteArgs) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    let config = RepoConfig::init_from_remote(&current_dir, &args.remote)?;

    println!("{}", config.to_pretty_string());

    Ok(())
}
