use std::path::Path;
use anyhow::anyhow;
use crate::io::fs::fs_readable::NamedFSReadable;
use crate::pack::pack_config::PackConfig;
use crate::pack::pack_user_settings::PackUserSettings;

pub fn launch_via_steam(repo_dir: &Path, pack_name: &str) -> anyhow::Result<()> {
    let mut pack_config = PackConfig::read_from_named(repo_dir, pack_name)?
        .ok_or(anyhow!("Config for pack {} not found", pack_name))?;

    let addons = get_addon_paths(&mut pack_config, repo_dir)?;

    let mut launch_url = String::from("steam://rungameid/107410// -nolauncher ");

    for param in pack_config.client_params {
        launch_url.push_str(&param);
        launch_url.push(' ');
    }

    let addons_combined = format!("\"-mod={}\"", addons.join(";"));
    
    launch_url.push_str(&format!("{}", urlencoding::encode(&addons_combined)));

    open::that(launch_url)?;

    Ok(())
}

fn get_addon_paths(config: &mut PackConfig, base_path: &Path) -> anyhow::Result<Vec<String>> {
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