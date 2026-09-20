use crate::util::dirs::steam_install::{STEAM_FLATPAK_ID, SteamFlavour, SteamInstall};
use std::path::{Path, PathBuf};

/// Everything the user has to do once, before launching works.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinuxLaunchSetup {
    pub flavour: SteamFlavour,
    pub arma_install_dir: String,
    /// What goes in Arma 3's Steam launch options, or `None` when Steam needs
    /// nothing set up at all — which is the common case.
    pub launch_options: Option<String>,
    /// Broad host roots that need a Flatpak filesystem grant. Empty when native,
    /// or when every mod already lives in the sandbox's home.
    pub flatpak_roots: Vec<String>,
    /// The `flatpak override` command granting `flatpak_roots`, when there are any.
    pub flatpak_override_command: Option<String>,
    /// Roots that go into `PRESSURE_VESSEL_FILESYSTEMS_RW`.
    pub pressure_vessel_roots: Vec<String>,
}

/// Takes the Steam install and its libraries rather than an `ArmaInstall`, so
/// that this stays free of the Linux-only VDF parsing and has a single
/// definition on every platform.
pub fn linux_launch_setup(
    steam: &SteamInstall,
    arma_install_dir: &Path,
    libraries: &[PathBuf],
    mod_roots: &[PathBuf],
) -> LinuxLaunchSetup {
    let roots = collapse_roots(mod_roots);

    let pressure_vessel_roots: Vec<PathBuf> = roots
        .iter()
        .filter(|root| steam.needs_pressure_vessel_share(root, libraries))
        .cloned()
        .collect();

    let flatpak_roots: Vec<PathBuf> = roots
        .iter()
        .filter(|root| steam.needs_flatpak_grant(root))
        .cloned()
        .collect();

    LinuxLaunchSetup {
        flavour: steam.flavour(),
        arma_install_dir: arma_install_dir.display().to_string(),
        launch_options: launch_options(&pressure_vessel_roots),
        flatpak_override_command: flatpak_override_command(&flatpak_roots),
        flatpak_roots: to_strings(&flatpak_roots),
        pressure_vessel_roots: to_strings(&pressure_vessel_roots),
    }
}

/// The string the user pastes into Steam, if any.
///
/// Its only job is to share filesystem roots with pressure-vessel. Every launch
/// flag, `-noLauncher` included, travels in the preset file instead, so that
/// nothing about a launch depends on the user having pasted correctly.
///
/// `%command%` is required because an environment assignment precedes it: Steam
/// treats a launch-options string containing it as a template. With no roots to
/// share there is no assignment, and therefore nothing to paste.
fn launch_options(pressure_vessel_roots: &[PathBuf]) -> Option<String> {
    if pressure_vessel_roots.is_empty() {
        return None;
    }

    let roots = pressure_vessel_roots
        .iter()
        .map(|root| root.display().to_string())
        .collect::<Vec<_>>()
        .join(":");

    Some(format!("PRESSURE_VESSEL_FILESYSTEMS_RW={roots} %command%"))
}

/// The grant command, for the user to run. Never executed by pamm.
///
/// `--filesystem` is additive across invocations, so this does not disturb
/// grants the user already has.
fn flatpak_override_command(flatpak_roots: &[PathBuf]) -> Option<String> {
    if flatpak_roots.is_empty() {
        return None;
    }

    let filesystems = flatpak_roots
        .iter()
        .map(|root| format!("--filesystem={}", root.display()))
        .collect::<Vec<_>>()
        .join(" ");

    Some(format!(
        "flatpak override --user {filesystems} {STEAM_FLATPAK_ID}"
    ))
}

/// Drops any root already contained in another, so a grant is never issued
/// twice for the same tree.
fn collapse_roots(mod_roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = mod_roots.to_vec();

    roots.sort();
    roots.dedup();

    // Sorted, so any root nested inside another follows it directly or after
    // its siblings; a linear scan keeping only the outermost is enough.
    let mut outermost: Vec<PathBuf> = Vec::new();
    for root in roots {
        if outermost.iter().any(|kept| root.starts_with(kept)) {
            continue;
        }
        outermost.push(root);
    }

    outermost
}

fn to_strings(paths: &[PathBuf]) -> Vec<String> {
    paths.iter().map(|p| p.display().to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::dirs::steam_install::SteamInstall;

    fn setup_for(
        flavour: SteamFlavour,
        install_dir: &str,
        libraries: &[&str],
        mods: &[&str],
    ) -> LinuxLaunchSetup {
        let root = match flavour {
            SteamFlavour::Native => "/home/bob/.local/share/Steam",
            SteamFlavour::Flatpak => {
                "/home/bob/.var/app/com.valvesoftware.Steam/.local/share/Steam"
            }
        };

        linux_launch_setup(
            &SteamInstall::new(PathBuf::from(root), flavour, PathBuf::from("/home/bob")),
            Path::new(install_dir),
            &libraries.iter().map(PathBuf::from).collect::<Vec<_>>(),
            &mods.iter().map(PathBuf::from).collect::<Vec<_>>(),
        )
    }

    #[test]
    fn collapse_roots_keeps_only_the_outermost() {
        let roots = collapse_roots(&[
            PathBuf::from("/mnt/games/FPArma"),
            PathBuf::from("/mnt/games/FPArma/externals"),
            PathBuf::from("/opt/mods"),
        ]);

        assert_eq!(
            roots,
            vec![
                PathBuf::from("/mnt/games/FPArma"),
                PathBuf::from("/opt/mods"),
            ]
        );
    }

    // Component-wise containment, not string prefix: /mnt/ab is not inside
    // /mnt/a and must keep its own grant.
    #[test]
    fn collapse_roots_does_not_confuse_sibling_names() {
        let roots = collapse_roots(&[PathBuf::from("/mnt/a"), PathBuf::from("/mnt/ab")]);

        assert_eq!(
            roots,
            vec![PathBuf::from("/mnt/a"), PathBuf::from("/mnt/ab")]
        );
    }

    // Native Steam with mods in $HOME: nothing to share, nothing to grant, so
    // no environment assignment and therefore no %command% either.
    #[test]
    fn native_with_mods_in_home_needs_no_env_assignment() {
        let setup = setup_for(
            SteamFlavour::Native,
            "/home/bob/.local/share/Steam/steamapps/common/Arma 3",
            &["/home/bob/.local/share/Steam"],
            &["/home/bob/FPArma"],
        );

        assert_eq!(setup.launch_options, None);
        assert!(setup.pressure_vessel_roots.is_empty());
        assert!(setup.flatpak_roots.is_empty());
        assert_eq!(setup.flatpak_override_command, None);
    }

    #[test]
    fn native_with_mods_in_a_steam_library_needs_no_env_assignment() {
        let setup = setup_for(
            SteamFlavour::Native,
            "/mnt/steamlib/steamapps/common/Arma 3",
            &["/home/bob/.local/share/Steam", "/mnt/steamlib"],
            &["/mnt/steamlib/FPArma"],
        );

        assert_eq!(setup.launch_options, None);
        assert!(setup.pressure_vessel_roots.is_empty());
    }

    #[test]
    fn native_with_mods_on_another_drive_shares_the_root() {
        let setup = setup_for(
            SteamFlavour::Native,
            "/home/bob/.local/share/Steam/steamapps/common/Arma 3",
            &["/home/bob/.local/share/Steam"],
            &["/mnt/games/FPArma"],
        );

        assert_eq!(
            setup.launch_options.as_deref(),
            Some("PRESSURE_VESSEL_FILESYSTEMS_RW=/mnt/games/FPArma %command%")
        );
        // Native has no Flatpak layer at all.
        assert_eq!(setup.flatpak_override_command, None);
    }

    #[test]
    fn flatpak_with_mods_in_the_persisted_home_needs_nothing() {
        let setup = setup_for(
            SteamFlavour::Flatpak,
            "/home/bob/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/Arma 3",
            &["/home/bob/.var/app/com.valvesoftware.Steam/.local/share/Steam"],
            &["/home/bob/.var/app/com.valvesoftware.Steam/FPArma"],
        );

        assert_eq!(setup.launch_options, None);
        assert_eq!(setup.flatpak_override_command, None);
    }

    // Spec §3.3 row 2: the Flatpak grant is needed, but no share — once granted,
    // the path lands under the sandbox's $HOME, which is shared by default.
    #[test]
    fn flatpak_with_mods_elsewhere_in_home_needs_a_grant_but_no_share() {
        let setup = setup_for(
            SteamFlavour::Flatpak,
            "/home/bob/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/Arma 3",
            &["/home/bob/.var/app/com.valvesoftware.Steam/.local/share/Steam"],
            &["/home/bob/FPArma"],
        );

        assert_eq!(setup.flatpak_roots, vec!["/home/bob/FPArma".to_string()]);
        assert_eq!(
            setup.flatpak_override_command.as_deref(),
            Some("flatpak override --user --filesystem=/home/bob/FPArma com.valvesoftware.Steam")
        );
        assert!(setup.pressure_vessel_roots.is_empty());
        assert_eq!(setup.launch_options, None);
    }

    #[test]
    fn flatpak_with_mods_on_another_drive_needs_both_layers() {
        let setup = setup_for(
            SteamFlavour::Flatpak,
            "/home/bob/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/Arma 3",
            &["/home/bob/.var/app/com.valvesoftware.Steam/.local/share/Steam"],
            &["/mnt/games/FPArma", "/var/lib/armamods"],
        );

        assert_eq!(
            setup.launch_options.as_deref(),
            Some("PRESSURE_VESSEL_FILESYSTEMS_RW=/mnt/games/FPArma:/var/lib/armamods %command%")
        );
        assert_eq!(
            setup.flatpak_override_command.as_deref(),
            Some(
                "flatpak override --user --filesystem=/mnt/games/FPArma --filesystem=/var/lib/armamods com.valvesoftware.Steam"
            )
        );
    }
}
