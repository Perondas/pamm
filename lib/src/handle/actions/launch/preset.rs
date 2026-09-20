use anyhow::Context;
use std::fs::write;
use std::path::{Path, PathBuf};

/// Prefix on generated preset files, so they are recognisable among whatever
/// else lives in the game directory.
const PRESET_PREFIX: &str = "pamm_";

/// Mandatory on every launch, vanilla included: Arma's .NET launcher fails
/// under Proton with a `SteamLayerWrap` assembly error and stops the game
/// starting.
const NO_LAUNCHER: &str = "-noLauncher";

/// Turns a pack name into a preset filename.
///
/// Restricted to `[A-Za-z0-9._-]` so that the `-par=` value stays a bare
/// relative filename needing no percent-encoding in the `steam://` URL. A pack
/// called `My Mods (v2)` would otherwise break the URL.
pub(crate) fn preset_file_name(pack_name: &str) -> String {
    let sanitised: String = pack_name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-') {
                c
            } else {
                '_'
            }
        })
        .collect();

    format!("{PRESET_PREFIX}{sanitised}.txt")
}

/// Renders a preset: one startup parameter per line.
///
/// Every launch flag lives here, [`NO_LAUNCHER`] included, so that a launch
/// depends on nothing the user had to type themselves.
///
/// One `-mod=` per line rather than a single combined directive, which avoids
/// Arma's Linux semicolon-escaping entirely. Values are always quoted because
/// mod paths may contain spaces.
pub(crate) fn render_preset(params: &[String], arma_mod_paths: &[String]) -> String {
    let mut lines: Vec<String> = vec![NO_LAUNCHER.to_string()];

    lines.extend(params.iter().cloned());
    lines.extend(arma_mod_paths.iter().map(|path| format!("-mod=\"{path}\"")));

    format!("{}\n", lines.join("\n"))
}

/// Writes a preset into the Arma install directory and returns its full path.
///
/// Presets live directly in the game directory so the `-par` value stays a bare
/// relative filename, which Arma resolves against that directory.
pub(crate) fn write_preset(
    arma_install_dir: &Path,
    file_name: &str,
    body: &str,
) -> anyhow::Result<PathBuf> {
    let path = arma_install_dir.join(file_name);

    log::debug!("Writing preset to {path:?}");
    write(&path, body).with_context(|| format!("Failed to write preset to {path:#?}"))?;

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_utils::TestTempDir;

    #[test]
    fn clean_pack_names_pass_through() {
        assert_eq!(preset_file_name("main"), "pamm_main.txt");
        assert_eq!(preset_file_name("core-v1.2_a"), "pamm_core-v1.2_a.txt");
    }

    // Anything that would need escaping in the URL is replaced, not dropped, so
    // two different packs cannot collapse onto the same preset by accident.
    #[test]
    fn awkward_pack_names_are_sanitised() {
        assert_eq!(preset_file_name("My Mods (v2)"), "pamm_My_Mods__v2_.txt");
        assert_eq!(preset_file_name("a/b"), "pamm_a_b.txt");
    }

    #[test]
    fn renders_one_parameter_per_line_with_quoted_mods() {
        let rendered = render_preset(
            &["-name=FPArma".to_string()],
            &[
                r"Z:\mnt\games\FPArma\@CBA_A3".to_string(),
                r"Z:\mnt\games\FPArma\@ace".to_string(),
            ],
        );

        assert_eq!(
            rendered,
            "-noLauncher\n-name=FPArma\n-mod=\"Z:\\mnt\\games\\FPArma\\@CBA_A3\"\n-mod=\"Z:\\mnt\\games\\FPArma\\@ace\"\n"
        );
    }

    // Vanilla is just a preset with an empty mod list — no special-casing in the
    // launch path. It still carries -noLauncher, which every launch needs.
    #[test]
    fn renders_a_vanilla_preset_without_mods() {
        assert_eq!(render_preset(&[], &[]), "-noLauncher\n");
        assert_eq!(
            render_preset(&["-skipIntro".to_string()], &[]),
            "-noLauncher\n-skipIntro\n"
        );
    }

    #[test]
    fn writes_the_preset_into_the_game_directory() {
        let dir = TestTempDir::new("pamm_write_preset");

        let path = write_preset(&dir.0, "pamm_main.txt", "-skipIntro\n").unwrap();

        assert_eq!(path, dir.0.join("pamm_main.txt"));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "-skipIntro\n");
    }

    #[test]
    fn overwrites_a_stale_preset() {
        let dir = TestTempDir::new("pamm_overwrite_preset");

        write_preset(&dir.0, "pamm_main.txt", "-old\n").unwrap();
        let path = write_preset(&dir.0, "pamm_main.txt", "-new\n").unwrap();

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "-new\n");
    }
}
