use flutter_rust_bridge::for_generated::anyhow;

/// The state of Arma's Proton prefix on Linux.
///
/// Like `linux_setup`, declared and generated on every platform so the
/// committed Dart bindings are identical whichever machine ran the codegen;
/// only the bodies are platform-specific.
pub struct ProtonInfo {
    /// `None` when `config_info` could not be read.
    pub version: Option<String>,
    pub proton_dir: String,
    /// `None` when TeamSpeak is not installed in the prefix.
    pub teamspeak_exe: Option<String>,
}

#[cfg(target_os = "linux")]
fn detect() -> anyhow::Result<pamm_lib::util::proton::proton_env::Proton> {
    use anyhow::Context;

    let arma = pamm_lib::util::dirs::arma_install::find_arma_install()
        .context("Failed to find the Arma 3 installation")?;
    pamm_lib::util::proton::proton_env::Proton::detect(&arma)
}

/// The child is Proton's wrapper script. Reaping it on a thread keeps it from
/// lingering as a zombie for as long as the UI runs.
#[cfg(target_os = "linux")]
fn reap(mut child: std::process::Child) {
    std::thread::spawn(move || child.wait());
}

#[cfg(target_os = "linux")]
pub fn proton_info() -> anyhow::Result<ProtonInfo> {
    let proton = detect()?;

    Ok(ProtonInfo {
        version: proton.version().ok(),
        proton_dir: proton.proton_dir.display().to_string(),
        teamspeak_exe: pamm_lib::util::proton::teamspeak::find_teamspeak(&proton.prefix())
            .map(|exe| exe.display().to_string()),
    })
}

/// Blocks until the installer exits.
#[cfg(target_os = "linux")]
pub fn proton_install_teamspeak(installer: String, silent: bool) -> anyhow::Result<()> {
    pamm_lib::util::proton::teamspeak::install_teamspeak(
        &detect()?,
        std::path::Path::new(&installer),
        silent,
    )
}

#[cfg(target_os = "linux")]
pub fn proton_launch_teamspeak() -> anyhow::Result<()> {
    use anyhow::Context;
    use pamm_lib::util::proton::teamspeak::{find_teamspeak, launch_teamspeak};

    let proton = detect()?;
    let exe = find_teamspeak(&proton.prefix()).context("TeamSpeak is not installed")?;

    reap(launch_teamspeak(&proton, &exe, None)?);
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn proton_open_explorer(path: Option<String>) -> anyhow::Result<()> {
    reap(detect()?.open_explorer(path.as_deref())?);
    Ok(())
}

/// Stops everything in the prefix, including the game.
#[cfg(target_os = "linux")]
pub fn proton_kill_all() -> anyhow::Result<()> {
    detect()?.kill_all()
}

#[cfg(not(target_os = "linux"))]
pub fn proton_info() -> anyhow::Result<ProtonInfo> {
    anyhow::bail!("proton_info is only available on Linux");
}

/// Blocks until the installer exits.
#[cfg(not(target_os = "linux"))]
pub fn proton_install_teamspeak(_: String, _: bool) -> anyhow::Result<()> {
    anyhow::bail!("proton_install_teamspeak is only available on Linux");
}

#[cfg(not(target_os = "linux"))]
pub fn proton_launch_teamspeak() -> anyhow::Result<()> {
    anyhow::bail!("proton_launch_teamspeak is only available on Linux");
}

#[cfg(not(target_os = "linux"))]
pub fn proton_open_explorer(_: Option<String>) -> anyhow::Result<()> {
    anyhow::bail!("proton_open_explorer is only available on Linux");
}

/// Stops everything in the prefix, including the game.
#[cfg(not(target_os = "linux"))]
pub fn proton_kill_all() -> anyhow::Result<()> {
    anyhow::bail!("proton_kill_all is only available on Linux");
}
