use clap::Args;
use pamm_lib::consts::{CONFIG_FILE_NAME, MANIFEST_FILE_NAME};
use pamm_lib::pack::pack_config::PackConfig;
use pamm_lib::pack::pack_manifest::PackManifest;
use std::env::current_dir;
use std::fs;

#[derive(Debug, Args)]
pub struct SyncPackArgs {}

pub fn sync_pack_command(args: SyncPackArgs) -> anyhow::Result<()> {
    let config_file = current_dir()?.join(CONFIG_FILE_NAME);
    let pack_file = current_dir()?.join(MANIFEST_FILE_NAME);

    let local_config: PackConfig = if config_file.exists() {
        let file = fs::File::open(&config_file)?;
        serde_json::from_reader(file)?
    } else {
        return Err(anyhow::anyhow!("config file does not exist"));
    };

    let local_manifest = if pack_file.exists() {
        let file = fs::File::open(&pack_file)?;
        serde_cbor::from_reader::<PackManifest, _>(file)?
    } else {
        return Err(anyhow::anyhow!("pack file does not exist"));
    };

    // TODO: add remote config sync

    let remote_manifest_url = local_config
        .remote
        .expect("Remote not set in config")
        .join(MANIFEST_FILE_NAME)?;

    let remote_manifest = serde_cbor::from_reader::<PackManifest, _>(
        ureq::get(remote_manifest_url.to_string())
            .call()?
            .body_mut()
            .as_reader(),
    )?;
    
    println!("Got remote manifest: {:?}", remote_manifest);

    Ok(())
}
