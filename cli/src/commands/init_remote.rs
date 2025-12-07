use clap::Args;
use pamm_lib::fs::fs_writable::KnownFSWritable;
use pamm_lib::fs::init_pack::init_pack_on_fs;
use pamm_lib::net::downloadable::KnownDownloadable;
use pamm_lib::pack::config::pack_config::PackConfig;
use pamm_lib::repo::remote_config::RemoteConfig;
use url::Url;

#[derive(Debug, Args)]
pub struct InitRemoteArgs {
    /// Force refresh all addons, ignoring cached state
    #[arg()]
    pub remote: Url,
}

pub fn init_remote_command(args: InitRemoteArgs) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    let config = PackConfig::download_known(&args.remote)?;

    init_pack_on_fs(&config, &current_dir)?;

    println!("Successfully initialized remote pack: {:?}", config);

    let remote_config = RemoteConfig::new(args.remote);

    remote_config.write_to_known(&current_dir)?;

    Ok(())
}
