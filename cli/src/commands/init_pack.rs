use std::env::current_dir;
use std::fs;
use std::path::PathBuf;
use clap::Args;
use pamm_lib::fs::part_reader::read_to_part;
use pamm_lib::pack::pack_manifest::PackConfig;
use pamm_lib::pack::pack_part::part::PackPart;
use crate::commands::input::from_cli_input::FromCliInput;
use crate::commands::macros::{check_is_dir, check_is_file};

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

    println!("Pack config: {:?}", pack_config);
    let required_addons = index_addon_folder(PathBuf::from(&pack_config.required_addons_path))?;
    let optional_addons = if let Some(optional_path) = &pack_config.optional_addons_path {
        index_addon_folder(PathBuf::from(optional_path))?
    } else {
        vec![]
    };
    
    

    Ok(())
}



fn index_addon_folder(fs_path: PathBuf) -> anyhow::Result<Vec<PackPart>> {
    let addon_folders = fs::read_dir(fs_path)?;

    let mut paths = vec![];
    for entry in addon_folders {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.file_name().unwrap().to_str().unwrap().to_owned();
        
        let part = read_to_part(path, path_str.as_str())?;
        paths.push(part);
    }
    Ok(paths)
}