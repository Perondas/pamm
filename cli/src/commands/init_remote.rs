use clap::Args;
use pamm_lib::fs::init_pack::init_pack_on_fs;
use pamm_lib::net::downloadable::KnownDownloadable;
use pamm_lib::pack::config::pack_config::PackConfig;
use url::Url;

#[derive(Debug, Args)]
pub struct InitRemoteArgs {
    /// Force refresh all addons, ignoring cached state
    #[arg()]
    pub remote: Url,
}

pub fn init_remote_command(args: InitRemoteArgs) -> anyhow::Result<()> {
    let remote_config = PackConfig::download_known(&args.remote)?;

    let config = remote_config.with_remote(args.remote);

    init_pack_on_fs(&config, &std::env::current_dir()?)?;

    println!("Successfully initialized remote pack: {:?}", config);

    Ok(())
}
