use anyhow::Context;
use std::path::PathBuf;

pub fn find_steam_dir() -> anyhow::Result<PathBuf> {
    let steam_path = steamlocate::locate()
        .context("Failed to locate Steam")?
        .path()
        .to_path_buf();

    log::debug!("Found Steam install path: {:?}", steam_path);

    Ok(steam_path)
}
