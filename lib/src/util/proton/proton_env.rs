//! Running Windows programs inside Arma's Proton prefix.
//!
//! Everything here goes through the same Proton build Arma uses, so that
//! programs started by pamm share the game's wineserver.

use crate::util::dirs::arma_install::ArmaInstall;
use crate::util::proton::paths::{
    compat_data_dir, proton_dir_from_config_info, proton_version, wine_prefix,
};
use anyhow::{Context, bail};
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};

/// Arma's Proton setup. Detected fresh on every use rather than cached, since
/// the user can switch Arma's Proton version at any time.
#[derive(Debug, Clone)]
pub struct Proton {
    /// The Proton installation, holding the `proton` script.
    pub proton_dir: PathBuf,
    /// `steamapps/compatdata/107410`, the parent of the Wine prefix.
    pub compat_data: PathBuf,
    pub steam_root: PathBuf,
}

impl Proton {
    /// Detects the Proton setup of an Arma install. Fails if Arma has never
    /// been launched through Steam, as the prefix does not exist yet.
    pub fn detect(arma: &ArmaInstall) -> anyhow::Result<Self> {
        let compat_data = compat_data_dir(&arma.install_dir)?;

        if !wine_prefix(&compat_data).is_dir() {
            bail!(
                "No Proton prefix at {compat_data:?}. Launch Arma 3 through Steam once to create it."
            );
        }

        let proton_dir = proton_dir_from_config_info(&compat_data, &arma.steam)?;

        Ok(Self {
            proton_dir,
            compat_data,
            steam_root: arma.steam.root().to_path_buf(),
        })
    }

    pub fn prefix(&self) -> PathBuf {
        wine_prefix(&self.compat_data)
    }

    pub fn version(&self) -> anyhow::Result<String> {
        proton_version(&self.compat_data)
    }

    /// `proton run <program> <args>`, with the environment Proton needs.
    pub fn command<S: AsRef<std::ffi::OsStr>>(
        &self,
        program: impl AsRef<std::ffi::OsStr>,
        args: &[S],
    ) -> Command {
        let mut command = Command::new(self.proton_dir.join("proton"));
        command
            .arg("run")
            .arg(program)
            .args(args)
            .env("STEAM_COMPAT_DATA_PATH", &self.compat_data)
            .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &self.steam_root);
        command
    }

    /// Runs a program and waits for it to exit.
    pub fn run<S: AsRef<std::ffi::OsStr>>(
        &self,
        program: impl AsRef<std::ffi::OsStr>,
        args: &[S],
    ) -> anyhow::Result<ExitStatus> {
        let mut command = self.command(program, args);
        log::debug!("Running {command:?}");
        command.status().context("Failed to run proton")
    }

    /// Starts a program and returns immediately, discarding its output.
    ///
    /// The child is Proton's wrapper script, not the Windows program, so
    /// killing it does not reliably stop the program. Use [`Self::kill_all`].
    pub fn spawn<S: AsRef<std::ffi::OsStr>>(
        &self,
        program: impl AsRef<std::ffi::OsStr>,
        args: &[S],
    ) -> anyhow::Result<Child> {
        let mut command = self.command(program, args);
        log::debug!("Spawning {command:?}");
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to spawn proton")
    }

    /// Opens a URL with whichever program the prefix has registered for it.
    pub fn open_url(&self, url: &str) -> anyhow::Result<Child> {
        self.spawn("start", &[url])
    }

    /// Opens Wine's file browser, optionally at a Windows path such as `C:\`
    /// or `Z:\home\...`.
    pub fn open_explorer(&self, start_dir: Option<&str>) -> anyhow::Result<Child> {
        self.spawn("explorer.exe", start_dir.as_slice())
    }

    /// Kills every program in the prefix, **including the game**.
    pub fn kill_all(&self) -> anyhow::Result<()> {
        let wineserver = ["files", "dist"]
            .iter()
            .map(|dir| self.proton_dir.join(dir).join("bin").join("wineserver"))
            .find(|path| path.is_file())
            .with_context(|| format!("No wineserver in {:?}", self.proton_dir))?;

        let status = Command::new(wineserver)
            .arg("-k")
            .env("WINEPREFIX", self.prefix())
            .status()
            .context("Failed to run wineserver")?;

        // wineserver -k exits non-zero when nothing was running, which is fine.
        log::debug!("wineserver -k exited with {status}");
        Ok(())
    }
}
