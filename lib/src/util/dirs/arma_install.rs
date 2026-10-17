use crate::util::dirs::ARMA_APP_ID;
use crate::util::dirs::steam_install::{SteamInstall, find_steam_installs};
use anyhow::{Context, bail};
use std::path::PathBuf;
use steamlocate::{Library, SteamDir};

#[derive(Debug, Clone)]
pub struct ArmaInstall {
    /// The Steam library that holds Arma.
    pub steam: SteamInstall,
    /// The Arma 3 installation itself, under `steamapps/common`.
    pub install_dir: PathBuf,
    /// Every library registered with this Steam install. pressure-vessel shares
    /// these by default, so mods inside one need no extra plumbing.
    pub libraries: Vec<PathBuf>,
}

/// Finds the Arma 3 installation, and the Steam install that owns it.
pub fn find_arma_install() -> anyhow::Result<ArmaInstall> {
    log::debug!("Attempting to find Arma install");

    let installs = find_steam_installs()?;
    let mut tried = Vec::new();

    for steam in installs {
        let libraries = match read_libraries(&steam) {
            Ok(libraries) => libraries,
            Err(e) => {
                log::debug!("Skipping Steam install at {:?}: {e:#}", steam.root());
                tried.push(format!("  {} (unreadable: {e})", steam.root().display()));
                continue;
            }
        };

        let Some(install_dir) = find_arma_in(&libraries)? else {
            log::debug!("No Arma in the Steam install at {:?}", steam.root());
            tried.push(format!("  {} (no Arma 3)", steam.root().display()));
            continue;
        };

        log::debug!("Resolved Arma install: {install_dir:?}");

        return Ok(ArmaInstall {
            steam,
            install_dir,
            libraries,
        });
    }

    bail!(
        "Arma 3 was not found in any Steam library. Looked in:\n{}",
        tried.join("\n")
    );
}

/// Every library folder registered with this Steam install.
fn read_libraries(steam: &SteamInstall) -> anyhow::Result<Vec<PathBuf>> {
    let paths = SteamDir::from_dir(steam.root())
        .context("Invalid Steam directory")?
        .library_paths()
        .context("Unable to read libraryfolders")?;

    Ok(paths.iter().map(|path| steam.host_path(path)).collect())
}

/// Arma's installation directory, from whichever library holds it.
fn find_arma_in(libraries: &[PathBuf]) -> anyhow::Result<Option<PathBuf>> {
    for path in libraries {
        // A library on an unmounted drive is not an error, it just isn't
        // where Arma is.
        let library = match Library::from_dir(path) {
            Ok(library) => library,
            Err(e) => {
                log::trace!("Skipping unreadable library at {path:?}: {e}");
                continue;
            }
        };

        if let Some(app) = library.app(ARMA_APP_ID) {
            let app = app.context("Failed to read Arma's appmanifest")?;
            return Ok(Some(library.resolve_app_dir(&app)));
        }
    }

    Ok(None)
}
