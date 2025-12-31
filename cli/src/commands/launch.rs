use anyhow::anyhow;
use anyhow::Result;
use clap::Args;
use pamm_lib::io::fs::fs_readable::NamedFSReadable;
use pamm_lib::pack::pack_config::PackConfig;
use pamm_lib::pack::pack_user_settings::PackUserSettings;
use std::env::current_dir;
use std::path::Path;

#[derive(Debug, Args)]
pub struct LaunchArgs {
    #[arg()]
    pub name: String,
}

pub fn launch_command(args: LaunchArgs) -> anyhow::Result<()> {
    let current_dir = current_dir()?;

    let mut pack_config = PackConfig::read_from_named(&current_dir, &args.name)?
        .ok_or(anyhow!("Config for pack {} not found", args.name))?;

    let addons = get_addon_paths(&mut pack_config, &current_dir)?;

    let mut launch_url = String::from("steam://rungameid/107410// -nolauncher ");

    for param in pack_config.client_params {
        launch_url.push_str(&param);
        launch_url.push(' ');
    }

    let addons_combined = format!("\"-mod={}\"", addons.join(";"));

    println!("Mods to load: {}", addons_combined);

    launch_url.push_str(&format!("{}", urlencoding::encode(&addons_combined)));

    println!("Launching Arma 3 with URL:");
    println!("{}", launch_url);

    open::that(launch_url)?;

    Ok(())
}

fn get_addon_paths(config: &mut PackConfig, base_path: &Path) -> Result<Vec<String>> {
    let user_settings = PackUserSettings::read_from_named(base_path, &config.name)?
        .ok_or(anyhow!("User settings for pack {} not found", config.name))?;

    config.remove_disabled_optionals(&user_settings);

    let own_addons = config.get_addon_paths(base_path);

    if let Some(parent_name) = &config.parent {
        let mut parent_config = PackConfig::read_from_named(base_path, parent_name)?
            .ok_or(anyhow!("Config for parent pack {} not found", parent_name))?;

        Ok(own_addons
            .into_iter()
            .chain(get_addon_paths(&mut parent_config, base_path)?)
            .collect())
    } else {
        Ok(own_addons)
    }
}
