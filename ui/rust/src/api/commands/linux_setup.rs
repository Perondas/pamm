use flutter_rust_bridge::for_generated::anyhow;

/// The one-time Steam setup a repo needs on Linux.
///
/// Declared and generated on every platform so the committed Dart bindings
/// under `ui/lib/src/rust` are identical whichever machine ran the codegen;
/// only the body is platform-specific.
pub struct LinuxSetupInfo {
    pub steam_flavour: SteamFlavour,
    pub arma_install_dir: String,
    /// Paste into Steam → Arma 3 → Properties → Launch Options. `None` when
    /// Steam needs nothing set up.
    pub launch_options: Option<String>,
    /// The roots embedded in `launch_options`, listed separately for display.
    pub pressure_vessel_roots: Vec<String>,
    /// Host paths needing a Flatpak grant.
    pub flatpak_roots: Vec<String>,
    /// The command granting `flatpak_roots`. `None` for native Steam, or when
    /// nothing sits outside the sandbox.
    pub flatpak_override_command: Option<String>,
}

pub enum SteamFlavour {
    Native,
    Flatpak,
}

/// `Ok(None)` on platforms that need no setup, so the Dart side can render
/// nothing without special-casing an error.
#[cfg(target_os = "linux")]
pub fn linux_setup_info(repo_dir: String) -> anyhow::Result<Option<LinuxSetupInfo>> {
    if !cfg!(target_os = "linux") {
        return Ok(None);
    }

    let handle = ClientRepoHandle::open(Path::new(&repo_dir))?;
    let setup = handle.linux_launch_setup()?;

    Ok(Some(LinuxSetupInfo {
        steam_flavour: match setup.flavour {
            pamm_lib::util::dirs::steam_install::SteamFlavour::Native => SteamFlavour::Native,
            pamm_lib::util::dirs::steam_install::SteamFlavour::Flatpak => SteamFlavour::Flatpak,
        },
        arma_install_dir: setup.arma_install_dir,
        launch_options: setup.launch_options,
        pressure_vessel_roots: setup.pressure_vessel_roots,
        flatpak_roots: setup.flatpak_roots,
        flatpak_override_command: setup.flatpak_override_command,
    }))
}

#[cfg(not(target_os = "linux"))]
pub fn linux_setup_info(_: String) -> anyhow::Result<Option<LinuxSetupInfo>> {
    anyhow::bail!("linux_setup_info is only available on Linux");
}
