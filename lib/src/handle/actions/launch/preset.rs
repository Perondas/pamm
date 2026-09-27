use anyhow::Context;
use std::fs::write;
use std::path::{Path, PathBuf};

const PRESET_PREFIX: &str = "pamm_";

pub(crate) struct Preset {
    pack_name: String,
    params: Vec<String>,
    arma_mod_paths: Vec<String>,
}

impl Preset {
    pub fn new(pack_name: String, params: Vec<String>, arma_mod_paths: Vec<String>) -> Self {
        Self {
            pack_name,
            params,
            arma_mod_paths,
        }
    }

    pub fn write_to(&self, path: impl AsRef<Path>) -> anyhow::Result<PathBuf> {
        let file_name = preset_file_name(&self.pack_name);
        let body = render_preset(&self.params, &self.arma_mod_paths);

        let path = path.as_ref().join(file_name);

        log::debug!("Writing preset to {path:?}");
        write(&path, body).with_context(|| format!("Failed to write preset to {path:#?}"))?;

        Ok(path)
    }
}

/// Cleans a pack name so that it does not cause trouble when passed via URL to steam
fn preset_file_name(pack_name: &str) -> String {
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

fn render_preset(params: &[String], arma_mod_paths: &[String]) -> String {
    let mut lines: Vec<String> = vec!["-noLauncher".to_string()];

    lines.extend(params.iter().cloned());
    lines.extend(arma_mod_paths.iter().map(|path| format!("-mod=\"{path}\"")));

    format!("{}\n", lines.join("\n"))
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

        let preset = Preset::new("main".to_string(), vec!["-skipIntro".to_string()], vec![]);
        let path = preset.write_to(&dir.0).unwrap();

        assert_eq!(path, dir.0.join("pamm_main.txt"));
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "-noLauncher\n-skipIntro\n"
        );
    }

    #[test]
    fn overwrites_a_stale_preset() {
        let dir = TestTempDir::new("pamm_overwrite_preset");

        Preset::new("main".to_string(), vec!["-old".to_string()], vec![])
            .write_to(&dir.0)
            .unwrap();
        let path = Preset::new("main".to_string(), vec!["-new".to_string()], vec![])
            .write_to(&dir.0)
            .unwrap();

        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "-noLauncher\n-new\n"
        );
    }
}
