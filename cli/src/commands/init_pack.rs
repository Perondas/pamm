use std::env::current_dir;
use std::path::PathBuf;
use clap::Args;
use pamm_lib::pack::pack_manifest::PackConfig;
use crate::commands::input::from_cli_input::FromCliInput;
use crate::commands::macros::{check_is_dir, check_is_file};

#[derive(Debug, Args)]
pub struct InitPackArgs {
   // pub pack_path: PathBuf
}

pub fn init_pack_command(args: InitPackArgs) -> anyhow::Result<()> {
/*    let InitPackArgs { pack_path } = args;

    if (pack_path.is_dir()) {
        anyhow::bail!("Pack path must be a file path, not a directory");
    }*/

    let pack_file = current_dir()?.join("pack.pamm");

    if pack_file.exists() {
        anyhow::bail!("Pack already initialized at this location");
    }

    let pack_config = PackConfig::from_cli_input()?;

    println!("Pack config: {:?}", pack_config);



    Ok(())
}
