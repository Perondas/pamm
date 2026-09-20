use crate::util::dirs::steam_install::{SteamInstall, find_steam_installs};
use anyhow::{Context, anyhow, bail};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use steam_vdf_parser::{Value, Vdf, parse_text};

/// Arma 3's Steam app ID.
pub const ARMA_APP_ID: &str = "107410";

/// An Arma installation, together with the Steam that owns it. Which Steam it is
/// matters as much as where the game sits: it decides how paths we hand to the
/// game have to be spelled.
#[derive(Debug)]
pub struct ArmaInstall {
    /// Host path of `<library>/steamapps/common/<installdir>`. Absolute, because
    /// it is built from the library path in `libraryfolders.vdf`, which Steam
    /// stores absolute. Not canonicalized.
    pub install_dir: PathBuf,
    pub steam: SteamInstall,
}

/// Finds the Arma 3 installation, searching every Steam install on the machine.
pub fn find_arma_install() -> anyhow::Result<ArmaInstall> {
    log::debug!("Attempting to find Arma install directory");

    let steam_installs = find_steam_installs()?;
    let mut searched = Vec::new();

    for steam in steam_installs {
        searched.push(steam.root.clone());

        match arma_in_steam_install(&steam) {
            Ok(Some(install_dir)) => {
                log::debug!("Resolved full Arma install path: {install_dir:?}");
                return Ok(ArmaInstall { install_dir, steam });
            }
            Ok(None) => log::debug!("Arma is not installed under {:?}", steam.root),
            // One broken install must not hide the game in another.
            Err(e) => log::warn!("Could not search Steam install {:?}: {e:#}", steam.root),
        }
    }

    let searched = searched
        .iter()
        .map(|p| format!("  {}", p.display()))
        .collect::<Vec<_>>()
        .join("\n");
    bail!("Arma 3 was not found in any Steam library. Searched these Steam installs:\n{searched}")
}

/// The Arma install directory below one Steam install, if the game is there.
fn arma_in_steam_install(steam: &SteamInstall) -> anyhow::Result<Option<PathBuf>> {
    let libraryfolders_path = steam.library_folders_path();
    if !libraryfolders_path.is_file() {
        log::trace!("No libraryfolders.vdf at {libraryfolders_path:?}");
        return Ok(None);
    }

    log::trace!("Reading libraryfolders from {libraryfolders_path:?}");
    let libraryfolders_file = read_to_string(&libraryfolders_path)
        .context(anyhow!("Unable to read libraryfolders from path"))?;
    let libraryfolders =
        parse_text(&libraryfolders_file).context(anyhow!("Failed to parse libraryfolders"))?;

    for library in library_paths(&libraryfolders) {
        // Flatpak Steam writes this file from inside its sandbox, so the paths in
        // it are the sandbox's, not ours.
        let library = steam.to_host_path(&library);

        if !library.join("steamapps").is_dir() {
            log::trace!("Skipping {library:?}, it has no steamapps directory");
            continue;
        }

        if let Some(install_dir) = arma_in_library(&library)? {
            log::debug!("Found Arma in Steam library {library:?}");
            return Ok(Some(install_dir));
        }
    }

    Ok(None)
}

/// Every library path listed in `libraryfolders.vdf`.
///
/// Two formats exist: the current one maps an index to an object with a `path`,
/// the legacy one maps an index straight to the path. Entries that are neither
/// (the numeric pairs inside `apps` blocks, for instance) are dropped by the
/// caller, which keeps only paths that actually hold a `steamapps` directory.
fn library_paths(libraryfolders: &Vdf) -> Vec<PathBuf> {
    let Some(entries) = libraryfolders.as_obj() else {
        log::warn!("libraryfolders is not an object");
        return Vec::new();
    };

    entries
        .iter()
        .filter_map(|(_, entry)| match entry {
            Value::Obj(_) => entry.get("path").and_then(Value::as_str),
            Value::Str(path) => Some(path.as_ref()),
            _ => None,
        })
        .map(PathBuf::from)
        .collect()
}

/// The Arma install directory inside one Steam library, if the game is there.
fn arma_in_library(library: &Path) -> anyhow::Result<Option<PathBuf>> {
    let appmanifest_path = library
        .join("steamapps")
        .join(format!("appmanifest_{ARMA_APP_ID}.acf"));

    if !appmanifest_path.is_file() {
        return Ok(None);
    }

    log::trace!("Reading appmanifest from {appmanifest_path:?}");
    let appmanifest_file = read_to_string(&appmanifest_path)
        .context(anyhow!("Unable to read appmanifest from path"))?;
    let appmanifest =
        parse_text(&appmanifest_file).context(anyhow!("Failed to parse appmanifest"))?;

    let install_dir_name = appmanifest
        .as_obj()
        .context(anyhow!("appmanifest is not an object"))?
        .get("installdir")
        .context(anyhow!("appmanifest does not contain 'installdir'"))?
        .as_str()
        .context(anyhow!("installdir is not a string"))?;

    log::debug!("Found Arma directory name in appmanifest: {install_dir_name:?}");

    let common = library.join("steamapps").join("common");
    let install_dir = common.join(install_dir_name);
    if install_dir.is_dir() {
        return Ok(Some(install_dir));
    }

    // `installdir` does not always match the directory Steam actually created,
    // and a manifest can outlive a manually deleted game folder. Neither is an
    // error: the game is simply not installed here.
    match find_ignoring_case(&common, install_dir_name) {
        Some(install_dir) => {
            log::debug!("Matched {install_dir_name:?} case-insensitively to {install_dir:?}");
            Ok(Some(install_dir))
        }
        None => {
            log::warn!(
                "{appmanifest_path:?} claims Arma is installed, but {install_dir:?} does not exist"
            );
            Ok(None)
        }
    }
}

fn find_ignoring_case(dir: &Path, name: &str) -> Option<PathBuf> {
    std::fs::read_dir(dir)
        .ok()?
        .filter_map(Result::ok)
        .find(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .eq_ignore_ascii_case(name)
        })
        .map(|entry| entry.path())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_utils::TestTempDir;
    use std::fs::{create_dir_all, write};

    #[test]
    fn library_paths_are_read_from_the_current_format() {
        let vdf = parse_text(
            r#""libraryfolders"
            {
                "0"
                {
                    "path"          "/home/user/.local/share/Steam"
                    "apps"          { "107410"  "46030747718" }
                }
                "1"
                {
                    "path"          "/mnt/games/SteamLibrary"
                }
            }"#,
        )
        .unwrap();

        assert_eq!(
            library_paths(&vdf),
            vec![
                PathBuf::from("/home/user/.local/share/Steam"),
                PathBuf::from("/mnt/games/SteamLibrary"),
            ]
        );
    }

    #[test]
    fn library_paths_are_read_from_the_legacy_format() {
        let vdf = parse_text(
            r#""LibraryFolders"
            {
                "TimeNextStatsReport"   "1234567890"
                "1"                     "/mnt/games"
            }"#,
        )
        .unwrap();

        // `TimeNextStatsReport` looks exactly like a library entry here; it is
        // dropped later, when the path turns out to hold no steamapps directory.
        assert_eq!(
            library_paths(&vdf),
            vec![PathBuf::from("1234567890"), PathBuf::from("/mnt/games")]
        );
    }

    fn library_with_manifest(tmp: &TestTempDir, install_dir_name: &str) -> PathBuf {
        let library = tmp.path().join("SteamLibrary");
        create_dir_all(library.join("steamapps/common")).unwrap();
        write(
            library.join(format!("steamapps/appmanifest_{ARMA_APP_ID}.acf")),
            format!("\"AppState\"\n{{\n\t\"installdir\"\t\"{install_dir_name}\"\n}}\n"),
        )
        .unwrap();
        library
    }

    #[test]
    fn arma_is_found_through_its_appmanifest() {
        let tmp = TestTempDir::new("pamm_arma_install_found");
        let library = library_with_manifest(&tmp, "Arma 3");
        create_dir_all(library.join("steamapps/common/Arma 3")).unwrap();

        assert_eq!(
            arma_in_library(&library).unwrap(),
            Some(library.join("steamapps/common/Arma 3"))
        );
    }

    // Steam's `installdir` casing does not always match the directory on disk.
    #[test]
    fn an_install_dir_with_different_casing_is_still_found() {
        let tmp = TestTempDir::new("pamm_arma_install_casing");
        let library = library_with_manifest(&tmp, "arma 3");
        create_dir_all(library.join("steamapps/common/Arma 3")).unwrap();

        assert_eq!(
            arma_in_library(&library).unwrap(),
            Some(library.join("steamapps/common/Arma 3"))
        );
    }

    // A manifest left behind by a manually deleted game means "not installed
    // here", not "broken".
    #[test]
    fn a_stale_appmanifest_reports_arma_as_missing() {
        let tmp = TestTempDir::new("pamm_arma_install_stale");
        let library = library_with_manifest(&tmp, "Arma 3");

        assert_eq!(arma_in_library(&library).unwrap(), None);
    }

    #[test]
    fn a_library_without_arma_is_skipped() {
        let tmp = TestTempDir::new("pamm_arma_install_absent");
        let library = tmp.path().join("SteamLibrary");
        create_dir_all(library.join("steamapps/common")).unwrap();

        assert_eq!(arma_in_library(&library).unwrap(), None);
    }
}
