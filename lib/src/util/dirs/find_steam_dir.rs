use anyhow::{Context, ensure};
use std::path::PathBuf;

pub fn find_steam_dir() -> anyhow::Result<PathBuf> {
    let steam_path = windows_registry::LOCAL_MACHINE
        .open(r"SOFTWARE\Valve\Steam")
        .map(|key| {
            key.get_string("InstallPath")
                .map(PathBuf::from)
                .context("Steam InstallPath not found in registry")
        })??;

    log::debug!("Found Steam install path in registry: {:?}", steam_path);

    ensure!(
        steam_path.exists(),
        "Steam install path does not exist at expected path: {:?}",
        steam_path
    );

    Ok(steam_path)
}
