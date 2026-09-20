use anyhow::{Context, anyhow, bail};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use steam_vdf_parser::{Value, parse_text};

/// Returns the absolute path to the Arma 3 installation directory,
/// `<steam library>/steamapps/common/<installdir>`. It is absolute because it
/// is built from the Steam library path in `libraryfolders.vdf`, which Steam
/// stores as an absolute path. The path is not canonicalized.
pub fn find_arma_install_dir() -> anyhow::Result<PathBuf> {
    log::debug!("Attempting to find Arma install directory");

    let libfolders_path = find_libraryfolders()?;

    log::trace!("Reading libraryfolders from {:?}", libfolders_path);
    let libfolders_file = read_to_string(&libfolders_path)
        .context(anyhow!("Unable to read libraryfolders from path"))?;

    let libfolders =
        parse_text(&libfolders_file).context(anyhow!("Failed to parse libraryfolders"))?;

    let install_location = libfolders
        .as_obj()
        .context(anyhow!("libraryfolders is not an object"))?
        .iter()
        .map(|(_, value)| arma_in_location(value).map(|contains_arma| (contains_arma, value)))
        .filter_map(|result| match result {
            Ok((true, value)) => Some(value),
            _ => None,
        })
        // I don't think that steam allows more than one install location per app, so we should be good
        .next()
        .context(anyhow!("Arma 3 not found in any library folder"))?;

    let library_path = install_location
        .get("path")
        .context(anyhow!("libraryfolders entry does not contain 'path'"))?
        .as_str()
        .map(PathBuf::from)
        .context(anyhow!("libraryfolders path is not a string"))?;

    log::debug!(
        "Found Steam library path containing Arma: {:?}",
        library_path
    );

    let appmanifest_path = PathBuf::from(&library_path).join("steamapps/appmanifest_107410.acf");
    log::trace!("Reading appmanifest from {:?}", appmanifest_path);
    let appmanifest_file = read_to_string(&appmanifest_path)
        .context(anyhow!("Unable to read appmanifest from path"))?;
    let appmanifest =
        parse_text(&appmanifest_file).context(anyhow!("Failed to parse appmanifest"))?;

    let arma_dir_name = appmanifest
        .as_obj()
        .context(anyhow!("appmanifest is not an object"))?
        .get("installdir")
        .context(anyhow!("appmanifest does not contain 'installdir'"))?
        .as_str()
        .map(PathBuf::from)
        .context(anyhow!("installdir is not a string"))?;

    log::debug!(
        "Found Arma directory name in appmanifest: {:?}",
        arma_dir_name
    );

    let full_path = library_path
        .join("steamapps")
        .join("common")
        .join(&arma_dir_name);

    log::debug!("Resolved full Arma install path: {:?}", full_path);

    Ok(full_path)
}

fn arma_in_location(value: &Value) -> anyhow::Result<bool> {
    let apps = value
        .get("apps")
        .context(anyhow!("libraryfolders entry does not contain 'apps'"))?;
    Ok(apps.get("107410").map(|_| true).unwrap_or_default())
}

/// Steam roots relative to $HOME, in order of preference.
const STEAM_ROOT_CANDIDATES: &[&str] = &[
    ".steam/root",
    ".steam/steam",
    ".local/share/Steam",
    ".var/app/com.valvesoftware.Steam/.local/share/Steam", // Flatpak
    ".var/app/com.valvesoftware.Steam/data/Steam",         // older Flatpak
    "snap/steam/common/.local/share/Steam",                // Snap
];

fn candidate_manifests() -> anyhow::Result<Vec<PathBuf>> {
    let home_dir = std::env::home_dir().ok_or_else(|| anyhow!("Unable to find home directory"))?;
    log::trace!("Found home directory: {home_dir:?}");

    let mut roots = Vec::new();

    // Explicit overrides win.
    if let Some(root) = std::env::var_os("STEAM_ROOT") {
        roots.push(PathBuf::from(root));
    }
    if let Some(data_home) = std::env::var_os("XDG_DATA_HOME") {
        roots.push(Path::new(&data_home).join("Steam"));
    }
    roots.extend(STEAM_ROOT_CANDIDATES.iter().map(|rel| home_dir.join(rel)));

    Ok(roots
        .into_iter()
        .map(|root| root.join("steamapps").join("libraryfolders.vdf"))
        .collect())
}

/// Every `libraryfolders.vdf` on this machine, deduplicated.
pub fn find_all_libraryfolders() -> anyhow::Result<Vec<PathBuf>> {
    let candidates = candidate_manifests()?;
    let mut found = Vec::new();
    let mut seen = HashSet::new();

    for path in &candidates {
        if !path.is_file() {
            log::trace!("No libraryfolders.vdf at {path:?}");
            continue;
        }
        // ~/.steam/root, ~/.steam/steam and ~/.local/share/Steam are usually
        // symlinks onto the same directory, so canonicalize before deduping.
        let key = path.canonicalize().unwrap_or_else(|_| path.clone());
        if seen.insert(key) {
            log::debug!("Found libraryfolders.vdf at {path:?}");
            found.push(path.clone());
        } else {
            log::trace!("Skipping {path:?}, duplicate of an earlier candidate");
        }
    }

    if found.is_empty() {
        let tried = candidates
            .iter()
            .map(|p| format!("  {}", p.display()))
            .collect::<Vec<_>>()
            .join("\n");
        bail!("No libraryfolders.vdf found in any known Steam location. Tried:\n{tried}");
    }

    Ok(found)
}

/// The first `libraryfolders.vdf` found.
pub fn find_libraryfolders() -> anyhow::Result<PathBuf> {
    find_all_libraryfolders().map(|mut found| found.remove(0))
}
