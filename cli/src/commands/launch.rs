use anyhow::anyhow;
use clap::Args;
use pamm_lib::io::fs::fs_readable::NamedFSReadable;
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
    let addons = get_addon_paths(&pack_config, &current_dir)?;

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
    config: &PackConfig,
    base_path: &PathBuf,
) -> anyhow::Result<Box<dyn Iterator<Item = String>>> {
    let stored_index = config.read_index_from_fs(base_path)?;

    let res = stored_index.get_addon_paths(base_path);

    println!("Loaded addons for pack {}:", config.name);

    if let Some(parent_name) = &config.parent {
        let parent_config = PackConfig::read_from_named(base_path, parent_name)?
            .ok_or(anyhow!("Config for parent pack {} not found", parent_name))?;
        Ok(Box::new(
            res.into_iter().chain(get_addon_paths(&parent_config, base_path)?),
        ))
    } else {
        Ok(Box::new(res.into_iter()))
    }
}


