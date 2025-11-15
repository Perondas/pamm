use clap::Args;
use pamm_lib::consts::CONFIG_FILE_NAME;
use pamm_lib::pack::pack_config::PackConfig;
use url::Url;

#[derive(Debug, Args)]
pub struct InitRemoteArgs {
    /// Force refresh all addons, ignoring cached state
    #[arg(short, long)]
    pub remote: Url,
}

pub fn init_remote_command(args: InitRemoteArgs) -> anyhow::Result<()> {
    let remote_config_url = args.remote.join(CONFIG_FILE_NAME)?;

    let remote_config = ureq::get(remote_config_url.to_string())
        .call()?
        .body_mut()
        .read_json::<PackConfig>()?;

    let config = PackConfig {
        remote: Some(args.remote),
        ..remote_config
    };


    config.init_on_disk( &std::env::current_dir()?)?;

    println!("Successfully initialized remote pack: {:?}", config);

    Ok(())
}
