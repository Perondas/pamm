use clap::Args;
use pamm_lib::consts::CONFIG_FILE_NAME;
use pamm_lib::pack::pack_manifest::PackConfig;
use reqwest::Url;

#[derive(Debug, Args)]
pub struct InitRemoteArgs {
    /// Force refresh all addons, ignoring cached state
    #[arg(short, long)]
    pub remote: Url,
}

pub fn init_remote_command(args: InitRemoteArgs) -> anyhow::Result<()> {
    let remote_config_url = args.remote.join(CONFIG_FILE_NAME)?;

    let remote_config_resp = reqwest::blocking::get(remote_config_url)?;
    if !remote_config_resp.status().is_success() {
        return Err(anyhow::anyhow!("Failed to fetch remote config"));
    }
    let remote_config: PackConfig = remote_config_resp.json()?;

    let config = PackConfig {
        remote: Some(args.remote.as_str().to_owned()),
        ..remote_config
    };

    let dir_path = std::env::current_dir()?.join(&config.name);
    std::fs::create_dir_all(&dir_path)?;
    config.init_on_disk(&dir_path)?;

    println!("Successfully initialized remote config at path: {:?}", dir_path);

    Ok(())
}
