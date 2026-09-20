use anyhow::{anyhow, bail};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// The Flatpak application ID of the Flathub Steam package.
pub const STEAM_FLATPAK_ID: &str = "com.valvesoftware.Steam";

/// How Steam is packaged, which decides whether Steam's view of the filesystem
/// is the same as ours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SteamFlavour {
    /// Installed on the host (RPM/DEB/AUR/...). Steam sees the paths we see.
    Native,
    /// Flathub's `com.valvesoftware.Steam`. It runs with `--persist=.`, which
    /// bind-mounts `~/.var/app/com.valvesoftware.Steam/` over `$HOME` inside the
    /// sandbox, so one directory has two absolute paths: ours and Steam's. The
    /// real home is not visible inside the sandbox at all.
    Flatpak,
    /// The Snap package. Its `$HOME` is remapped as well, but unlike Flatpak the
    /// real home stays visible, so host paths keep resolving and we treat it
    /// like a native install.
    Snap,
}

/// A Steam installation found on this machine.
#[derive(Debug, Clone)]
pub struct SteamInstall {
    /// The Steam root as *we* see it, e.g.
    /// `~/.var/app/com.valvesoftware.Steam/.local/share/Steam`.
    pub root: PathBuf,
    pub flavour: SteamFlavour,
    /// The host directory Steam sees as `$HOME`, set only when that differs
    /// from ours (i.e. for Flatpak). Everything below it is visible to the game
    /// for free, because pressure-vessel shares Steam's `$HOME` into the Proton
    /// container; everything outside it needs a Flatpak filesystem grant.
    sandbox_home: Option<PathBuf>,
    /// Our home directory, which is also the prefix Steam uses for its own
    /// `$HOME` when recording paths (Flatpak keeps the path spelling and only
    /// changes what is mounted there).
    home: PathBuf,
}

impl SteamInstall {
    /// Describes the install rooted at `root`, given the home directory Steam
    /// records its own paths against.
    pub fn new(root: PathBuf, flavour: SteamFlavour, home: PathBuf) -> Self {
        Self {
            root,
            flavour,
            sandbox_home: match flavour {
                SteamFlavour::Flatpak => Some(flatpak_app_dir(&home)),
                SteamFlavour::Native | SteamFlavour::Snap => None,
            },
            home,
        }
    }

    pub fn library_folders_path(&self) -> PathBuf {
        self.root.join("steamapps").join("libraryfolders.vdf")
    }

    /// The host directory Steam sees as its `$HOME`, if that is not simply our
    /// own home directory.
    pub fn sandbox_home(&self) -> Option<&Path> {
        self.sandbox_home.as_deref()
    }

    /// Translates a path as Steam recorded it (in `libraryfolders.vdf` and
    /// friends) into a path this process can open. Under Flatpak, Steam writes
    /// `/home/<user>/.local/share/Steam`, which on the host is really
    /// `~/.var/app/com.valvesoftware.Steam/.local/share/Steam`.
    pub fn to_host_path(&self, steam_path: &Path) -> PathBuf {
        match (&self.sandbox_home, steam_path.strip_prefix(&self.home)) {
            (Some(sandbox_home), Ok(rest)) => sandbox_home.join(rest),
            _ => steam_path.to_path_buf(),
        }
    }

    /// The inverse of [`Self::to_host_path`]: a host path spelled the way Steam
    /// (and anything it launches) sees it. Used for symlink targets that have to
    /// be written as absolute paths.
    pub fn to_steam_path(&self, host_path: &Path) -> PathBuf {
        match &self.sandbox_home {
            Some(sandbox_home) => match host_path.strip_prefix(sandbox_home) {
                Ok(rest) => self.home.join(rest),
                Err(_) => host_path.to_path_buf(),
            },
            None => host_path.to_path_buf(),
        }
    }

    /// Whether `host_path` is inside the tree Steam sees as its `$HOME`. Such
    /// paths reach the game with no further setup; the catch is that they are
    /// spelled differently on each side of the sandbox.
    pub fn is_inside_sandbox_home(&self, host_path: &Path) -> bool {
        match &self.sandbox_home {
            Some(sandbox_home) => host_path.starts_with(sandbox_home),
            // Nothing is remapped, so the question does not arise.
            None => false,
        }
    }

    /// Whether the game can only reach `host_path` if the user has granted
    /// Flatpak Steam access to it.
    pub fn requires_flatpak_grant(&self, host_path: &Path) -> bool {
        self.flavour == SteamFlavour::Flatpak && !self.is_inside_sandbox_home(host_path)
    }
}

/// Steam roots relative to `$HOME`, in order of preference.
const STEAM_ROOT_CANDIDATES: &[&str] = &[
    ".steam/root",
    ".steam/steam",
    ".local/share/Steam",
    ".var/app/com.valvesoftware.Steam/.local/share/Steam", // Flatpak
    ".var/app/com.valvesoftware.Steam/data/Steam",         // older Flatpak
    "snap/steam/common/.local/share/Steam",                // Snap
];

fn candidate_roots(home: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();

    // Explicit overrides win.
    if let Some(root) = std::env::var_os("STEAM_ROOT") {
        roots.push(PathBuf::from(root));
    }
    if let Some(data_home) = std::env::var_os("XDG_DATA_HOME") {
        roots.push(Path::new(&data_home).join("Steam"));
    }
    roots.extend(STEAM_ROOT_CANDIDATES.iter().map(|rel| home.join(rel)));

    roots
}

/// Classifies a root by where it sits, so that an explicit `$STEAM_ROOT`
/// pointing into the Flatpak tree is still recognised as sandboxed.
fn flavour_of(root: &Path, home: &Path) -> SteamFlavour {
    if root.starts_with(flatpak_app_dir(home)) {
        SteamFlavour::Flatpak
    } else if root.starts_with(home.join("snap/steam")) {
        SteamFlavour::Snap
    } else {
        SteamFlavour::Native
    }
}

/// `~/.var/app/com.valvesoftware.Steam`, the host side of Flatpak Steam's
/// `$HOME`.
pub fn flatpak_app_dir(home: &Path) -> PathBuf {
    home.join(".var/app").join(STEAM_FLATPAK_ID)
}

pub fn home_dir() -> anyhow::Result<PathBuf> {
    #[allow(deprecated)]
    std::env::home_dir().ok_or_else(|| anyhow!("Unable to find home directory"))
}

/// Every Steam installation on this machine, in order of preference.
///
/// `~/.steam/root`, `~/.steam/steam` and `~/.local/share/Steam` are usually
/// symlinks onto the same directory, so roots are canonicalized before being
/// deduplicated; otherwise every game would be reported several times.
pub fn find_steam_installs() -> anyhow::Result<Vec<SteamInstall>> {
    let home = home_dir()?;
    log::trace!("Found home directory: {home:?}");

    let candidates = candidate_roots(&home);
    let mut found: Vec<SteamInstall> = Vec::new();
    let mut seen = HashSet::new();

    for root in &candidates {
        if !root.join("steamapps").is_dir() {
            log::trace!("No steamapps directory below {root:?}");
            continue;
        }

        let key = root.canonicalize().unwrap_or_else(|_| root.clone());
        if !seen.insert(key) {
            log::trace!("Skipping {root:?}, duplicate of an earlier candidate");
            continue;
        }

        let flavour = flavour_of(root, &home);
        log::debug!("Found {flavour:?} Steam install at {root:?}");
        found.push(SteamInstall::new(root.clone(), flavour, home.clone()));
    }

    if found.is_empty() {
        let tried = candidates
            .iter()
            .map(|p| format!("  {}", p.display()))
            .collect::<Vec<_>>()
            .join("\n");
        bail!("No Steam installation found in any known location. Tried:\n{tried}");
    }

    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flatpak_install() -> SteamInstall {
        let home = PathBuf::from("/home/user");
        SteamInstall::new(
            flatpak_app_dir(&home).join(".local/share/Steam"),
            SteamFlavour::Flatpak,
            home,
        )
    }

    fn native_install() -> SteamInstall {
        SteamInstall::new(
            PathBuf::from("/home/user/.local/share/Steam"),
            SteamFlavour::Native,
            PathBuf::from("/home/user"),
        )
    }

    // Flatpak Steam writes its config from inside the sandbox, where the Steam
    // root is below `/home/<user>`. On the host that same directory lives under
    // `~/.var/app`, and opening the path Steam recorded would fail.
    #[test]
    fn a_flatpak_path_below_home_is_translated_to_the_app_dir() {
        let steam = flatpak_install();

        assert_eq!(
            steam.to_host_path(Path::new("/home/user/.local/share/Steam")),
            Path::new("/home/user/.var/app/com.valvesoftware.Steam/.local/share/Steam")
        );
    }

    // A library on another drive is outside the remapped home, so both sides
    // spell it the same way.
    #[test]
    fn a_flatpak_path_outside_home_is_left_alone() {
        let steam = flatpak_install();
        let library = Path::new("/mnt/games/SteamLibrary");

        assert_eq!(steam.to_host_path(library), library);
        assert_eq!(steam.to_steam_path(library), library);
    }

    #[test]
    fn host_and_steam_paths_round_trip_under_flatpak() {
        let steam = flatpak_install();
        let steam_side = Path::new("/home/user/.local/share/Steam/steamapps/common/Arma 3");

        let host_side = steam.to_host_path(steam_side);
        assert!(steam.is_inside_sandbox_home(&host_side));
        assert_eq!(steam.to_steam_path(&host_side), steam_side);
    }

    // Nothing is remapped for a native install, so paths must survive untouched
    // and nothing ever needs a Flatpak grant.
    #[test]
    fn a_native_install_never_rewrites_paths() {
        let steam = native_install();
        let library = Path::new("/home/user/.local/share/Steam");

        assert_eq!(steam.to_host_path(library), library);
        assert_eq!(steam.to_steam_path(library), library);
        assert!(!steam.requires_flatpak_grant(Path::new("/mnt/games")));
    }

    // The repo living outside the sandbox home is the case that needs a grant;
    // one inside it is reachable for free.
    #[test]
    fn only_paths_outside_the_flatpak_home_need_a_grant() {
        let steam = flatpak_install();

        assert!(steam.requires_flatpak_grant(Path::new("/home/user/git/pamm_repo")));
        assert!(!steam.requires_flatpak_grant(Path::new(
            "/home/user/.var/app/com.valvesoftware.Steam/pamm_repo"
        )));
    }

    #[test]
    fn roots_are_classified_by_where_they_sit() {
        let home = Path::new("/home/user");

        assert_eq!(
            flavour_of(&flatpak_app_dir(home).join("data/Steam"), home),
            SteamFlavour::Flatpak
        );
        assert_eq!(
            flavour_of(&home.join("snap/steam/common/.local/share/Steam"), home),
            SteamFlavour::Snap
        );
        assert_eq!(
            flavour_of(&home.join(".local/share/Steam"), home),
            SteamFlavour::Native
        );
    }
}
