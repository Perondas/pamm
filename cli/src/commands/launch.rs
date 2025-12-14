use anyhow::anyhow;
use clap::{Args, arg};
use pamm_lib::fs::fs_readable::NamedFSReadable;
use pamm_lib::manifest::pack_manifest::PackManifest;
use pamm_lib::pack::pack_config::PackConfig;
use std::env::current_dir;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct LaunchArgs {
    #[arg()]
    pub name: String,
}

pub fn launch_command(args: LaunchArgs) -> anyhow::Result<()> {
    let current_dir = current_dir()?;

    let pack_config = PackConfig::read_from_named(&current_dir, &args.name)?
        .ok_or(anyhow!("Config for pack {} not found", args.name))?;
    let addons = get_addon_paths(&args.name, &current_dir)?;

    let mut launch_url = String::from("steam://rungameid/107410// -nolauncher ");

    for param in pack_config.client_params {
        launch_url.push_str(&param);
        launch_url.push(' ');
    }

    let addons_combined = addons.collect::<Vec<_>>().join(";");

    println!("Mods to load: {}", addons_combined);

    launch_url.push_str(&format!("-mod={}", urlencoding::encode(&addons_combined)));

    println!("Launching Arma 3 with URL:");
    println!("{}", launch_url);

    open::that(launch_url)?;

    Ok(())
}

fn get_addon_paths(
    name: &str,
    base_path: &PathBuf,
) -> anyhow::Result<Box<dyn Iterator<Item = String>>> {
    let local_manifest = PackManifest::read_from_named(base_path, name)?
        .ok_or(anyhow!("Manifest for pack {} not found", name))?;
    let pack_config = PackConfig::read_from_named(base_path, name)?
        .ok_or(anyhow!("Config for pack {} not found", name))?;

    let res = local_manifest
        .get_addon_paths(base_path)?
        .into_iter()
        .map(clean_path);

    println!("Loaded addons for pack {}:", name);

    if let Some(parent_name) = pack_config.parent {
        Ok(Box::new(
            res.chain(get_addon_paths(&parent_name, base_path)?),
        ))
    } else {
        Ok(Box::new(res.into_iter()))
    }
}

#[cfg(target_os = "windows")]
fn clean_path(path: PathBuf) -> String {
    path.to_str()
        .expect("mods must be UTF-8")
        .strip_prefix("\\\\?\\")
        .unwrap()
        .to_string()
}

#[cfg(target_os = "linux")]
fn clean_path(path: PathBuf) -> String {
    path.to_str().expect("mods must be UTF-8").to_string()
}
