use crate::util::dirs::steam_install::{SteamInstall, find_steam_installs};
use anyhow::{Context, anyhow, bail};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use steam_vdf_parser::{Value, parse_text};

pub const ARMA_APP_ID: &str = "107410";

#[derive(Debug, Clone)]
pub struct ArmaInstall {
    /// The Steam library that holds Arma.
    pub steam: SteamInstall,
    /// `<library>/steamapps/common/<installdir>`, absolute but not canonicalized.
    pub library_path: PathBuf,
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

        let Some(library_path) = libraries
            .iter()
            .find(|(_, has_arma)| *has_arma)
            .map(|(path, _)| path.clone())
        else {
            log::debug!("No Arma in the Steam install at {:?}", steam.root());
            tried.push(format!("  {} (no Arma 3)", steam.root().display()));
            continue;
        };

        let install_dir = library_path
            .join("steamapps")
            .join("common")
            .join(read_install_dir_name(&library_path)?);

        log::debug!("Resolved Arma install: {install_dir:?}");

        return Ok(ArmaInstall {
            steam,
            library_path,
            install_dir,
            libraries: libraries.into_iter().map(|(path, _)| path).collect(),
        });
    }

    bail!(
        "Arma 3 was not found in any Steam library. Looked in:\n{}",
        tried.join("\n")
    );
}

/// Every library folder registered with this Steam install, each flagged with
/// whether it holds Arma.
fn read_libraries(steam: &SteamInstall) -> anyhow::Result<Vec<(PathBuf, bool)>> {
    let manifest_path = steam.library_folders_path();
    log::trace!("Reading libraryfolders from {manifest_path:?}");

    let manifest = read_to_string(&manifest_path)
        .context(anyhow!("Unable to read libraryfolders from path"))?;
    let manifest = parse_text(&manifest).context(anyhow!("Failed to parse libraryfolders"))?;

    manifest
        .as_obj()
        .context(anyhow!("libraryfolders is not an object"))?
        .iter()
        .map(|(_, entry)| {
            let path = entry
                .get("path")
                .context(anyhow!("libraryfolders entry does not contain 'path'"))?
                .as_str()
                .map(PathBuf::from)
                .context(anyhow!("libraryfolders path is not a string"))?;

            // Flatpak Steam records these from inside the sandbox, so they need
            // translating back before we can open them.
            Ok((steam.host_path(&path), arma_in_location(entry)))
        })
        .collect()
}

fn arma_in_location(entry: &Value) -> bool {
    entry
        .get("apps")
        .and_then(|apps| apps.get(ARMA_APP_ID))
        .is_some()
}

/// Arma's folder name under `steamapps/common`, from its app manifest.
fn read_install_dir_name(library_path: &Path) -> anyhow::Result<PathBuf> {
    let manifest_path = library_path
        .join("steamapps")
        .join(format!("appmanifest_{ARMA_APP_ID}.acf"));
    log::trace!("Reading appmanifest from {manifest_path:?}");

    let manifest =
        read_to_string(&manifest_path).context(anyhow!("Unable to read appmanifest from path"))?;
    let manifest = parse_text(&manifest).context(anyhow!("Failed to parse appmanifest"))?;

    manifest
        .as_obj()
        .context(anyhow!("appmanifest is not an object"))?
        .get("installdir")
        .context(anyhow!("appmanifest does not contain 'installdir'"))?
        .as_str()
        .map(PathBuf::from)
        .context(anyhow!("installdir is not a string"))
}
