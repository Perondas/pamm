use crate::util::dirs::ARMA_APP_ID;
use crate::util::dirs::steam_install::SteamInstall;
use anyhow::Context;
use std::path::{Path, PathBuf};

/// `<library>/steamapps/compatdata/107410`
pub fn compat_data_dir(arma_install_dir: &Path) -> anyhow::Result<PathBuf> {
    let steamapps = arma_install_dir
        .parent()
        .and_then(Path::parent)
        .with_context(|| format!("Unexpected Arma install location {arma_install_dir:?}"))?;

    Ok(steamapps.join("compatdata").join(ARMA_APP_ID.to_string()))
}

pub fn wine_prefix(compat_data: &Path) -> PathBuf {
    compat_data.join("pfx")
}

/// The Proton installation that set up the prefix.
///
/// Proton writes `config_info` into the compatdata directory: a version line,
/// followed by paths inside the Proton installation. Walking up from those to
/// the directory holding the `proton` script finds it.
pub fn proton_dir_from_config_info(
    compat_data: &Path,
    steam: &SteamInstall,
) -> anyhow::Result<PathBuf> {
    let config_info_path = compat_data.join("config_info");
    let config_info = std::fs::read_to_string(&config_info_path)
        .with_context(|| format!("Failed to read {config_info_path:?}"))?;

    let found = config_info
        .lines()
        .skip(1)
        .filter(|line| line.starts_with('/'))
        .flat_map(|line| {
            // Written by Steam, so under Flatpak these are sandbox paths.
            let path = steam.host_path(Path::new(line.trim()));
            path.ancestors().map(Path::to_path_buf).collect::<Vec<_>>()
        })
        .find(|dir| dir.join("proton").is_file());

    found.with_context(|| {
        format!(
            "The Proton build recorded in {config_info_path:?} no longer exists. \
             Launch Arma 3 through Steam once to update it."
        )
    })
}

/// The Proton version that set up the prefix, from the first line of
/// `config_info`.
pub fn proton_version(compat_data: &Path) -> anyhow::Result<String> {
    let config_info_path = compat_data.join("config_info");
    let config_info = std::fs::read_to_string(&config_info_path)
        .with_context(|| format!("Failed to read {config_info_path:?}"))?;

    config_info
        .lines()
        .next()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .with_context(|| format!("{config_info_path:?} is empty"))
}

/// Maps a Windows `C:\...` path to its location inside the prefix.
pub fn windows_to_prefix_path(prefix: &Path, windows_path: &str) -> Option<PathBuf> {
    let (drive, rest) = windows_path.split_once(":\\")?;
    if !drive.eq_ignore_ascii_case("c") {
        return None;
    }

    Some(
        rest.split('\\')
            .filter(|part| !part.is_empty())
            .fold(prefix.join("drive_c"), |path, part| path.join(part)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::dirs::steam_install::SteamFlavour;
    use crate::util::test_utils::TestTempDir;

    #[test]
    fn compat_data_sits_next_to_common() {
        assert_eq!(
            compat_data_dir(Path::new("/mnt/lib/steamapps/common/Arma 3")).unwrap(),
            PathBuf::from("/mnt/lib/steamapps/compatdata/107410")
        );
    }

    #[test]
    fn proton_dir_is_found_from_config_info() {
        let dir = TestTempDir::new("pamm_proton_config_info");
        let proton = dir.path().join("steamapps/common/Proton 9.0");
        std::fs::create_dir_all(proton.join("files/share/fonts")).unwrap();
        std::fs::write(proton.join("proton"), "").unwrap();

        let compat_data = dir.path().join("compatdata");
        std::fs::create_dir_all(&compat_data).unwrap();
        std::fs::write(
            compat_data.join("config_info"),
            format!("9.0-205\n{}/files/share/fonts/\n", proton.display()),
        )
        .unwrap();

        let steam = SteamInstall::new(
            dir.path().to_path_buf(),
            SteamFlavour::Native,
            PathBuf::from("/home/bob"),
        );

        assert_eq!(
            proton_dir_from_config_info(&compat_data, &steam).unwrap(),
            proton
        );
    }

    #[test]
    fn deleted_proton_dir_is_an_error() {
        let dir = TestTempDir::new("pamm_proton_config_info_deleted");
        std::fs::write(
            dir.path().join("config_info"),
            "9.0-205\n/nonexistent/Proton 9.0/files/share/fonts/\n",
        )
        .unwrap();

        let steam = SteamInstall::new(
            dir.path().to_path_buf(),
            SteamFlavour::Native,
            PathBuf::from("/home/bob"),
        );

        assert!(proton_dir_from_config_info(dir.path(), &steam).is_err());
    }

    #[test]
    fn maps_c_drive_into_the_prefix() {
        assert_eq!(
            windows_to_prefix_path(
                Path::new("/pfx"),
                r"C:\Program Files\TeamSpeak 3 Client\ts3client_win64.exe"
            ),
            Some(PathBuf::from(
                "/pfx/drive_c/Program Files/TeamSpeak 3 Client/ts3client_win64.exe"
            ))
        );
        assert_eq!(windows_to_prefix_path(Path::new("/pfx"), r"Z:\home"), None);
    }
}
