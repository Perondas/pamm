use crate::commands::input::from_cli_input::FromCliInput;
use clap::Args;
use pamm_lib::pack::pack_manifest::{PackConfig, PackManifest};
use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct InitPackArgs {
   // pub pack_path: PathBuf
}

pub fn init_pack_command(args: InitPackArgs) -> anyhow::Result<()> {
    let pack_file = current_dir()?.join("pack.pamm");

    if pack_file.exists() {
        anyhow::bail!("Pack already initialized at this location");
    }

    let pack_config = PackConfig::from_cli_input()?;

    let manifest = PackManifest::new(pack_config)?;
    let (fs_manifest, addon_parts) = manifest.into_fs_manifest();

    let file = fs::File::create(&pack_file)?;
    serde_json::to_writer_pretty(file, &fs_manifest)?;

    for part in addon_parts {
        let file_path = PathBuf::from(part.get_rel_path()).join("addon.pamm");
        let file = fs::File::create(file_path)?;
        serde_cbor::to_writer(file, &part)?;
    }

    Ok(())
}
