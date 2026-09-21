use anyhow::{anyhow, bail};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub const STEAM_FLATPAK_ID: &str = "com.valvesoftware.Steam";

/// How Steam is installed. This decides both how host paths map into the
/// container the game runs in and whether a Flatpak filesystem grant is needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SteamFlavour {
    Native,
    Flatpak,
}

/// One Steam installation found on this machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteamInstall {
    root: PathBuf,
    flavour: SteamFlavour,
    /// The real `$HOME`. Kept as a field rather than read from the environment
    /// on demand so that path translation stays a pure function and Flatpak
    /// behaviour is testable on a machine that has no Flatpak Steam.
    home: PathBuf,
}

impl SteamInstall {
    pub fn new(root: PathBuf, flavour: SteamFlavour, home: PathBuf) -> Self {
        Self {
            root,
            flavour,
            home,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn flavour(&self) -> SteamFlavour {
        self.flavour
    }

    pub fn library_folders_path(&self) -> PathBuf {
        self.root.join("steamapps").join("libraryfolders.vdf")
    }

    /// The host directory the sandbox presents to the game as `$HOME`.
    /// `~/.var/app/com.valvesoftware.Steam` under Flatpak, nothing when native.
    fn sandbox_home(&self) -> Option<PathBuf> {
        match self.flavour {
            SteamFlavour::Native => None,
            SteamFlavour::Flatpak => {
                Some(self.home.join(".var").join("app").join(STEAM_FLATPAK_ID))
            }
        }
    }

    /// Translates a host path into the path the game process sees.
    ///
    /// The two coincide everywhere except inside the Flatpak persisted home,
    /// which the sandbox mounts at `/home/<user>`. Getting this wrong is silent:
    /// the wrong path still exists inside the sandbox, it just holds something
    /// else.
    pub fn container_path(&self, host: &Path) -> PathBuf {
        let Some(sandbox_home) = self.sandbox_home() else {
            return host.to_path_buf();
        };

        match host.strip_prefix(&sandbox_home) {
            Ok(rest) => self.home.join(rest),
            Err(_) => host.to_path_buf(),
        }
    }

    /// Whether this path needs `flatpak override --filesystem` before the game
    /// can see it at all. Only Flatpak installs have this layer, and only for
    /// paths outside the persisted home.
    pub fn needs_flatpak_grant(&self, host: &Path) -> bool {
        match self.sandbox_home() {
            None => false,
            Some(sandbox_home) => !host.starts_with(&sandbox_home),
        }
    }

    /// Whether this path needs to be listed in `PRESSURE_VESSEL_FILESYSTEMS_RW`.
    ///
    /// pressure-vessel already shares the sandbox's `$HOME` and every registered
    /// Steam library, so those need nothing. Everything else does.
    pub fn needs_pressure_vessel_share(&self, host: &Path, libraries: &[PathBuf]) -> bool {
        let container = self.container_path(host);

        if container.starts_with(&self.home) {
            return false;
        }

        !libraries
            .iter()
            .any(|library| container.starts_with(self.container_path(library)))
    }

    /// Inverse of [`Self::container_path`]: a path as Steam recorded it,
    /// spelled the way we can open it.
    ///
    /// Flatpak Steam writes its config from inside the sandbox, so paths in
    /// `libraryfolders.vdf` read `/home/<user>/.local/share/Steam` — which on
    /// the host is the `.var/app` tree, and which on a machine that *also* has
    /// native Steam resolves to an entirely different install.
    ///
    /// The mapping is genuinely ambiguous, because a granted host path under
    /// `$HOME` keeps its spelling inside the sandbox. It is resolved by looking:
    /// the persisted-home reading wins when it exists on disk.
    pub fn host_path(&self, steam_path: &Path) -> PathBuf {
        let Some(sandbox_home) = self.sandbox_home() else {
            return steam_path.to_path_buf();
        };

        match steam_path.strip_prefix(&self.home) {
            Ok(rest) => {
                let persisted = sandbox_home.join(rest);
                if persisted.exists() {
                    persisted
                } else {
                    steam_path.to_path_buf()
                }
            }
            Err(_) => steam_path.to_path_buf(),
        }
    }
}

/// Steam roots relative to `$HOME`, in order of preference, paired with the
/// flavour that root implies.
const STEAM_ROOT_CANDIDATES: &[(&str, SteamFlavour)] = &[
    (".steam/root", SteamFlavour::Native),
    (".steam/steam", SteamFlavour::Native),
    (".local/share/Steam", SteamFlavour::Native),
    (
        ".var/app/com.valvesoftware.Steam/.local/share/Steam",
        SteamFlavour::Flatpak,
    ),
    (
        ".var/app/com.valvesoftware.Steam/data/Steam",
        SteamFlavour::Flatpak,
    ),
    // Snap is out of scope for the Flatpak/pressure-vessel handling below, but
    // it is still a place Steam can be, so keep finding it.
    ("snap/steam/common/.local/share/Steam", SteamFlavour::Native),
];

fn home_dir() -> anyhow::Result<PathBuf> {
    std::env::home_dir().ok_or_else(|| anyhow!("Unable to find home directory"))
}

/// Every Steam root candidate we know of, tagged with its flavour. Explicit
/// environment overrides win; their flavour is inferred from the path.
fn candidate_roots(home: &Path) -> Vec<(PathBuf, SteamFlavour)> {
    let flatpak_marker = home.join(".var").join("app").join(STEAM_FLATPAK_ID);
    let flavour_of = |root: &Path| {
        if root.starts_with(&flatpak_marker) {
            SteamFlavour::Flatpak
        } else {
            SteamFlavour::Native
        }
    };

    let mut roots: Vec<(PathBuf, SteamFlavour)> = Vec::new();

    if let Some(root) = std::env::var_os("STEAM_ROOT") {
        let root = PathBuf::from(root);
        let flavour = flavour_of(&root);
        roots.push((root, flavour));
    }
    if let Some(data_home) = std::env::var_os("XDG_DATA_HOME") {
        let root = Path::new(&data_home).join("Steam");
        let flavour = flavour_of(&root);
        roots.push((root, flavour));
    }

    roots.extend(
        STEAM_ROOT_CANDIDATES
            .iter()
            .map(|(rel, flavour)| (home.join(rel), *flavour)),
    );

    roots
}

/// Every Steam installation on this machine that has a `libraryfolders.vdf`,
/// deduplicated.
pub fn find_steam_installs() -> anyhow::Result<Vec<SteamInstall>> {
    let home = home_dir()?;
    log::trace!("Found home directory: {home:?}");

    let candidates = candidate_roots(&home);
    let mut found = Vec::new();
    let mut seen = HashSet::new();

    for (root, flavour) in &candidates {
        let manifest = root.join("steamapps").join("libraryfolders.vdf");
        if !manifest.is_file() {
            log::trace!("No libraryfolders.vdf at {manifest:?}");
            continue;
        }
        // ~/.steam/root, ~/.steam/steam and ~/.local/share/Steam are usually
        // symlinks onto the same directory, so canonicalize before deduping.
        let key = manifest.canonicalize().unwrap_or_else(|_| manifest.clone());
        if seen.insert(key) {
            log::debug!("Found {flavour:?} Steam install at {root:?}");
            found.push(SteamInstall::new(root.clone(), *flavour, home.clone()));
        } else {
            log::trace!("Skipping {root:?}, duplicate of an earlier candidate");
        }
    }

    if found.is_empty() {
        let tried = candidates
            .iter()
            .map(|(root, _)| format!("  {}", root.display()))
            .collect::<Vec<_>>()
            .join("\n");
        bail!("No Steam installation found in any known location. Tried:\n{tried}");
    }

    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_utils::TestTempDir;

    fn flatpak() -> SteamInstall {
        SteamInstall::new(
            PathBuf::from("/home/bob/.var/app/com.valvesoftware.Steam/.local/share/Steam"),
            SteamFlavour::Flatpak,
            PathBuf::from("/home/bob"),
        )
    }

    fn native() -> SteamInstall {
        SteamInstall::new(
            PathBuf::from("/home/bob/.local/share/Steam"),
            SteamFlavour::Native,
            PathBuf::from("/home/bob"),
        )
    }

    // The whole point of the flavour distinction: inside the Flatpak sandbox
    // /home/bob *is* the .var/app tree, so a path in the persisted home has a
    // different name to the game than it has to us.
    #[test]
    fn flatpak_remaps_the_persisted_home_onto_the_real_home() {
        assert_eq!(
            flatpak().container_path(Path::new(
                "/home/bob/.var/app/com.valvesoftware.Steam/FPArma/@ace"
            )),
            PathBuf::from("/home/bob/FPArma/@ace")
        );
    }

    #[test]
    fn flatpak_leaves_paths_outside_the_persisted_home_alone() {
        assert_eq!(
            flatpak().container_path(Path::new("/mnt/games/FPArma/@ace")),
            PathBuf::from("/mnt/games/FPArma/@ace")
        );
    }

    #[test]
    fn native_container_paths_are_host_paths() {
        for path in ["/mnt/games/FPArma/@ace", "/home/bob/FPArma/@ace"] {
            assert_eq!(
                native().container_path(Path::new(path)),
                PathBuf::from(path)
            );
        }
    }

    // Spec §3.3, row by row.
    #[test]
    fn flatpak_grants_are_needed_outside_the_persisted_home_only() {
        let steam = flatpak();
        assert!(!steam.needs_flatpak_grant(Path::new(
            "/home/bob/.var/app/com.valvesoftware.Steam/FPArma"
        )));
        assert!(steam.needs_flatpak_grant(Path::new("/home/bob/FPArma")));
        assert!(steam.needs_flatpak_grant(Path::new("/mnt/games/FPArma")));
    }

    #[test]
    fn native_never_needs_a_flatpak_grant() {
        let steam = native();
        assert!(!steam.needs_flatpak_grant(Path::new("/mnt/games/FPArma")));
        assert!(!steam.needs_flatpak_grant(Path::new("/home/bob/FPArma")));
    }

    #[test]
    fn pressure_vessel_shares_home_and_libraries_by_default() {
        let libraries = vec![PathBuf::from("/mnt/steamlib")];

        let steam = native();
        assert!(!steam.needs_pressure_vessel_share(Path::new("/home/bob/FPArma"), &libraries));
        assert!(!steam.needs_pressure_vessel_share(Path::new("/mnt/steamlib/FPArma"), &libraries));
        assert!(steam.needs_pressure_vessel_share(Path::new("/mnt/games/FPArma"), &libraries));

        let steam = flatpak();
        assert!(!steam.needs_pressure_vessel_share(
            Path::new("/home/bob/.var/app/com.valvesoftware.Steam/FPArma"),
            &libraries
        ));
        // A grant is needed here, but no share. Once granted,
        // the path keeps its spelling inside the sandbox, where it sits under
        // $HOME and is shared by default.
        assert!(!steam.needs_pressure_vessel_share(Path::new("/home/bob/FPArma"), &libraries));
        assert!(steam.needs_pressure_vessel_share(Path::new("/mnt/games/FPArma"), &libraries));
    }

    #[test]
    fn native_host_paths_are_steam_paths() {
        assert_eq!(
            native().host_path(Path::new("/home/bob/.local/share/Steam")),
            PathBuf::from("/home/bob/.local/share/Steam")
        );
    }

    // Flatpak Steam records its own library as /home/<user>/.local/share/Steam,
    // which on the host is the .var/app tree. Reading that verbatim would, on a
    // machine that also has native Steam, silently open the wrong install.
    #[test]
    fn flatpak_host_paths_resolve_back_into_the_persisted_home() {
        let dir = TestTempDir::new("pamm_flatpak_host_path");
        let home = dir.0.join("home/bob");
        let persisted = home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam");
        std::fs::create_dir_all(&persisted).unwrap();

        let steam = SteamInstall::new(persisted.clone(), SteamFlavour::Flatpak, home.clone());

        assert_eq!(steam.host_path(&home.join(".local/share/Steam")), persisted);
    }

    // A granted path under $HOME keeps its spelling inside the sandbox, so it
    // must not be dragged into the .var/app tree just because it starts with
    // the home prefix.
    #[test]
    fn flatpak_host_paths_leave_granted_home_paths_alone() {
        let dir = TestTempDir::new("pamm_flatpak_granted_path");
        let home = dir.0.join("home/bob");
        let granted = home.join("FPArma");
        std::fs::create_dir_all(&granted).unwrap();

        let steam = SteamInstall::new(
            home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam"),
            SteamFlavour::Flatpak,
            home.clone(),
        );

        assert_eq!(steam.host_path(&granted), granted);
    }

    #[test]
    fn candidate_roots_tag_flatpak_paths_as_flatpak() {
        let home = PathBuf::from("/home/bob");
        let candidates = candidate_roots(&home);

        let flatpak_root = home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam");
        let native_root = home.join(".local/share/Steam");

        assert_eq!(
            candidates
                .iter()
                .find(|(root, _)| *root == flatpak_root)
                .map(|(_, flavour)| *flavour),
            Some(SteamFlavour::Flatpak)
        );
        assert_eq!(
            candidates
                .iter()
                .find(|(root, _)| *root == native_root)
                .map(|(_, flavour)| *flavour),
            Some(SteamFlavour::Native)
        );
    }
}
