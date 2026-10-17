use crate::util::dirs::ARMA_APP_ID;
use anyhow::Context;
use std::path::PathBuf;

pub fn find_workshop_folder() -> anyhow::Result<PathBuf> {
    let steam_dir = steamlocate::locate().context("Failed to locate Steam")?;

    let (_, lib) = steam_dir
        .find_app(ARMA_APP_ID)
        .ok()
        .flatten()
        .context("Failed to find Arma 3 in Steam")?;

    let workshop_folder = lib
        .path()
        .join("steamapps")
        .join("workshop")
        .join("content")
        .join(ARMA_APP_ID.to_string())
        .canonicalize()
        .context("Failed to canonicalize workshop path")?;

    log::debug!("Found Arma 3 workshop folder: {:?}", workshop_folder);

    Ok(workshop_folder)
}
