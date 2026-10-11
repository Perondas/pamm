use crate::util::proton::paths::windows_to_prefix_path;
use crate::util::proton::proton_env::Proton;
use anyhow::{Context, bail};
use std::path::{Path, PathBuf};
use std::process::Child;

/// Where to get the installer. The Windows version is the one that runs in
/// the prefix.
pub const TEAMSPEAK_DOWNLOAD_URL: &str = "https://www.teamspeak.com/en/downloads";

/// TeamSpeak registers itself as the handler for these links on install.
const TS3_HANDLER_KEY: &str = r"ts3server\\shell\\open\\command";

/// Where TeamSpeak installs by default, machine-wide or per-user.
const FALLBACK_LOCATIONS: &[&str] = &[
    r"C:\Program Files\TeamSpeak 3 Client\ts3client_win64.exe",
    r"C:\Program Files (x86)\TeamSpeak 3 Client\ts3client_win32.exe",
    r"C:\users\steamuser\AppData\Local\TeamSpeak 3 Client\ts3client_win64.exe",
];

/// Runs the TeamSpeak installer inside the prefix and waits for it.
/// `silent` skips the NSIS wizard.
pub fn install_teamspeak(proton: &Proton, installer: &Path, silent: bool) -> anyhow::Result<()> {
    let args: &[&str] = if silent { &["/S"] } else { &[] };
    let status = proton.run(installer, args)?;

    if !status.success() {
        bail!("TeamSpeak installer failed with {status}");
    }
    Ok(())
}

/// The installed TeamSpeak executable, from the registry or else the default
/// install locations. The registry can lag briefly behind an install, as Wine
/// flushes it to disk with a delay.
pub fn find_teamspeak(prefix: &Path) -> Option<PathBuf> {
    let from_registry = ["system.reg", "user.reg"]
        .iter()
        .filter_map(|file| std::fs::read_to_string(prefix.join(file)).ok())
        .filter_map(|registry| ts3_handler_exe(&registry))
        .filter_map(|exe| windows_to_prefix_path(prefix, &exe));

    let fallback = FALLBACK_LOCATIONS
        .iter()
        .filter_map(|exe| windows_to_prefix_path(prefix, exe));

    from_registry.chain(fallback).find(|path| path.is_file())
}

/// Starts TeamSpeak, connecting to `server_url` (`ts3server://...`) if given.
pub fn launch_teamspeak(
    proton: &Proton,
    exe: &Path,
    server_url: Option<&str>,
) -> anyhow::Result<Child> {
    proton
        .spawn(exe, server_url.as_slice())
        .context("Failed to launch TeamSpeak")
}

/// The executable's Windows path, from the `ts3server://` handler in a Wine
/// `.reg` file.
fn ts3_handler_exe(registry: &str) -> Option<String> {
    let mut lines = registry.lines();

    lines.find(|line| {
        line.starts_with('[')
            && line
                .split_once(']')
                .is_some_and(|(key, _)| key.to_ascii_lowercase().ends_with(TS3_HANDLER_KEY))
    })?;

    let value = lines
        .take_while(|line| !line.starts_with('['))
        .find_map(|line| line.strip_prefix("@="))?;

    let command = unescape_reg_string(value)?;

    let exe = match command.strip_prefix('"') {
        Some(quoted) => quoted.split('"').next()?,
        None => command.split_whitespace().next()?,
    };

    Some(exe.to_string())
}

/// Unescapes a quoted `.reg` string value: `"C:\\a \"b\""` to `C:\a "b"`.
fn unescape_reg_string(value: &str) -> Option<String> {
    let inner = value.strip_prefix('"')?.strip_suffix('"')?;

    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            out.push(chars.next()?);
        } else {
            out.push(c);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_utils::TestTempDir;

    const SYSTEM_REG: &str = r#"WINE REGISTRY Version 2

[Software\\Classes\\ts3file\\shell\\open\\command] 1700000000
@="\"C:\\Wrong\\ts3client_win64.exe\" \"%1\""

[Software\\Classes\\ts3server\\shell\\open\\command] 1700000000
#time=1da1234567890ab
@="\"C:\\Program Files\\TeamSpeak 3 Client\\ts3client_win64.exe\" \"%1\""

[Software\\Classes\\ts3server\\DefaultIcon] 1700000000
@="C:\\Other.exe"
"#;

    #[test]
    fn reads_the_handler_exe_from_the_registry() {
        assert_eq!(
            ts3_handler_exe(SYSTEM_REG).as_deref(),
            Some(r"C:\Program Files\TeamSpeak 3 Client\ts3client_win64.exe")
        );
    }

    #[test]
    fn missing_handler_is_none() {
        assert_eq!(ts3_handler_exe("WINE REGISTRY Version 2\n"), None);
    }

    #[test]
    fn unquoted_command_takes_the_first_word() {
        let reg = "[Software\\\\Classes\\\\ts3server\\\\shell\\\\open\\\\command] 1\n@=\"C:\\\\ts3.exe %1\"\n";
        assert_eq!(ts3_handler_exe(reg).as_deref(), Some(r"C:\ts3.exe"));
    }

    #[test]
    fn falls_back_to_default_locations() {
        let dir = TestTempDir::new("pamm_find_teamspeak_fallback");
        let exe = dir
            .path()
            .join("drive_c/Program Files/TeamSpeak 3 Client/ts3client_win64.exe");
        std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
        std::fs::write(&exe, "").unwrap();

        assert_eq!(find_teamspeak(dir.path()), Some(exe));
    }

    #[test]
    fn prefers_the_registry() {
        let dir = TestTempDir::new("pamm_find_teamspeak_registry");
        let exe = dir.path().join("drive_c/Custom/ts3client_win64.exe");
        std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
        std::fs::write(&exe, "").unwrap();
        std::fs::write(
            dir.path().join("user.reg"),
            "[Software\\\\Classes\\\\ts3server\\\\shell\\\\open\\\\command] 1\n@=\"\\\"C:\\\\Custom\\\\ts3client_win64.exe\\\" \\\"%1\\\"\"\n",
        )
        .unwrap();

        assert_eq!(find_teamspeak(dir.path()), Some(exe));
    }
}
