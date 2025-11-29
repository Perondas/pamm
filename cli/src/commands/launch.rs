use clap::Args;
use pamm_lib::pack::pack_config::PackConfig;
use pamm_lib::pack::pack_manifest::PackManifest;
use std::env::current_dir;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct LaunchArgs {
    /*/// Force refresh all addons, ignoring cached state
    #[arg(short, long)]
    pub remote: Url,*/
}

pub fn launch_command(_args: LaunchArgs) -> anyhow::Result<()> {
    let local_config = PackConfig::read(&current_dir()?)?;
    let manifest = PackManifest::read(&current_dir()?)?;

    let mut launch_url = String::from("steam://rungameid/107410// -nolauncher ");

    for param in local_config.client_params {
        launch_url.push_str(&param);
        launch_url.push(' ');
    }

    let mods = manifest
        .get_required_addon_paths(&current_dir()?)?
        .into_iter()
        .chain(manifest.get_optional_addon_paths(&current_dir()?)?);

    let mods_combined = mods.map(clean_path).collect::<Vec<_>>().join(";");

    println!("Mods to load: {}", mods_combined);

    launch_url.push_str(&format!("-mod={}", urlencoding::encode(&mods_combined)));

    println!("Launching Arma 3 with URL:");
    println!("{}", launch_url);

    open::that(launch_url)?;

    Ok(())
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
