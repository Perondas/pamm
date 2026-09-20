use crate::util::dirs::steam_install::{STEAM_FLATPAK_ID, home_dir};
use std::path::{Path, PathBuf};
use std::process::Command;

/// What we could work out about Flatpak Steam's access to a host path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrantState {
    /// A filesystem grant covers the path; the game will be able to reach it.
    Granted,
    /// We read Steam's grants and none of them covers the path.
    Missing,
    /// We could not read the grants, or they contain entries we cannot resolve
    /// (`xdg-*` shortcuts, for instance). The path may or may not be reachable.
    Unknown,
}

/// Whether Flatpak Steam is allowed to read `path`.
///
/// Grants come from the app's own manifest plus any `flatpak override`, so we
/// ask `flatpak info --show-permissions` first and fall back to the override
/// files it writes. Both sources are merged: a grant from either is enough.
pub fn steam_grant_state(path: &Path) -> GrantState {
    let Ok(home) = home_dir() else {
        return GrantState::Unknown;
    };

    let mut entries = Vec::new();
    let mut read_anything = false;

    if let Some(permissions) = effective_permissions() {
        read_anything = true;
        entries.extend(parse_filesystems(&permissions));
    }
    for file in override_files(&home) {
        if let Ok(contents) = std::fs::read_to_string(&file) {
            log::trace!("Read Flatpak overrides from {file:?}");
            read_anything = true;
            entries.extend(parse_filesystems(&contents));
        }
    }

    if !read_anything {
        log::debug!("Could not read any Flatpak permissions for {STEAM_FLATPAK_ID}");
        return GrantState::Unknown;
    }

    log::trace!("Flatpak filesystem grants for {STEAM_FLATPAK_ID}: {entries:?}");

    let mut uncertain = false;
    let mut granted = false;

    for entry in entries {
        let (entry, revokes) = match entry.strip_prefix('!') {
            Some(rest) => (rest.to_string(), true),
            None => (entry, false),
        };

        match covers(&entry, path, &home) {
            // A revocation beats any grant, and Flatpak applies it the same way.
            Some(true) if revokes => return GrantState::Missing,
            Some(true) => granted = true,
            Some(false) => {}
            None => {
                log::debug!("Cannot resolve Flatpak filesystem grant '{entry}'");
                uncertain = true;
            }
        }
    }

    match (granted, uncertain) {
        (true, _) => GrantState::Granted,
        (false, true) => GrantState::Unknown,
        (false, false) => GrantState::Missing,
    }
}

/// The command that grants Flatpak Steam access to `path`, ready to be pasted
/// into a terminal. Deliberately never `--filesystem=home` or `=host`: the
/// Flathub Steam maintainers do not support exposing the whole home directory.
pub fn grant_command(path: &Path) -> String {
    format!(
        "flatpak override --user --filesystem={} {STEAM_FLATPAK_ID}\n  \
         flatpak kill {STEAM_FLATPAK_ID}",
        path.display()
    )
}

/// Steam's permissions after overrides are applied, as reported by the `flatpak`
/// CLI. `None` if it is not installed or the app is unknown to it.
fn effective_permissions() -> Option<String> {
    let output = Command::new("flatpak")
        .args(["info", "--show-permissions", STEAM_FLATPAK_ID])
        .output()
        .inspect_err(|e| log::debug!("Could not run `flatpak info`: {e}"))
        .ok()?;

    if !output.status.success() {
        log::debug!(
            "`flatpak info --show-permissions {STEAM_FLATPAK_ID}` failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
        return None;
    }

    String::from_utf8(output.stdout).ok()
}

fn override_files(home: &Path) -> Vec<PathBuf> {
    vec![
        home.join(".local/share/flatpak/overrides")
            .join(STEAM_FLATPAK_ID),
        PathBuf::from("/var/lib/flatpak/overrides").join(STEAM_FLATPAK_ID),
    ]
}

/// Pulls `filesystems=` out of the `[Context]` section of Flatpak's INI-shaped
/// permission format.
fn parse_filesystems(contents: &str) -> Vec<String> {
    let mut in_context = false;
    let mut entries = Vec::new();

    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_context = line.eq_ignore_ascii_case("[Context]");
            continue;
        }
        if !in_context {
            continue;
        }
        if let Some(value) = line.strip_prefix("filesystems=") {
            entries.extend(
                value
                    .split(';')
                    .map(str::trim)
                    .filter(|e| !e.is_empty())
                    .map(str::to_string),
            );
        }
    }

    entries
}

/// Whether a single grant covers `path`. `None` when the entry is a shortcut we
/// cannot resolve, so that the caller can stay honest about not knowing.
fn covers(entry: &str, path: &Path, home: &Path) -> Option<bool> {
    let entry = strip_mode(entry);

    match entry {
        "host" | "host-os" | "host-etc" => Some(true),
        "home" => Some(path.starts_with(home)),
        _ => {
            let granted = if let Some(rest) = entry.strip_prefix("~/") {
                home.join(rest)
            } else if entry.starts_with('/') {
                PathBuf::from(entry)
            } else {
                // `xdg-download`, `xdg-run/...` and friends: resolving these
                // properly means reading the user's XDG configuration, which we
                // do not do.
                return None;
            };
            Some(path.starts_with(granted))
        }
    }
}

/// Drops the `:ro`, `:rw` or `:create` suffix a grant may carry.
fn strip_mode(entry: &str) -> &str {
    match entry.rsplit_once(':') {
        Some((path, "ro" | "rw" | "create")) => path,
        _ => entry,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOME: &str = "/home/user";

    fn covers_path(entry: &str, path: &str) -> Option<bool> {
        covers(entry, Path::new(path), Path::new(HOME))
    }

    #[test]
    fn filesystems_are_read_from_the_context_section_only() {
        let contents = "[Application]\nname=com.valvesoftware.Steam\n\
                        \n[Context]\n\
                        shared=network;ipc;\n\
                        filesystems=/mnt/games;~/Mods:ro;xdg-run/app/x:create;\n\
                        \n[Environment]\nfilesystems=/not/a/grant;\n";

        assert_eq!(
            parse_filesystems(contents),
            vec!["/mnt/games", "~/Mods:ro", "xdg-run/app/x:create"]
        );
    }

    #[test]
    fn a_grant_covers_everything_below_it() {
        assert_eq!(
            covers_path("/mnt/games", "/mnt/games/pamm/repo"),
            Some(true)
        );
        assert_eq!(covers_path("/mnt/games", "/mnt/other"), Some(false));
        assert_eq!(covers_path("~/Mods", "/home/user/Mods/repo"), Some(true));
        assert_eq!(covers_path("/mnt/games:ro", "/mnt/games/repo"), Some(true));
        assert_eq!(covers_path("home", "/home/user/git/repo"), Some(true));
        assert_eq!(covers_path("host", "/anywhere"), Some(true));
    }

    // We would rather admit we do not know than claim a path is unreachable and
    // refuse to launch over it.
    #[test]
    fn xdg_shortcuts_are_reported_as_unresolvable() {
        assert_eq!(covers_path("xdg-download", "/home/user/Downloads"), None);
        assert_eq!(covers_path("xdg-data/Steam", "/home/user/.local"), None);
    }

    #[test]
    fn the_grant_command_names_the_exact_path() {
        let command = grant_command(Path::new("/home/user/git/repo"));

        assert!(command.contains("--filesystem=/home/user/git/repo"));
        assert!(command.contains(STEAM_FLATPAK_ID));
        // Never the blanket grants the Steam maintainers refuse to support.
        assert!(!command.contains("--filesystem=home"));
        assert!(!command.contains("--filesystem=host"));
    }
}
