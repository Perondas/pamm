use crate::handle::externals::external_addon::ExternalAddon;
use regex::Regex;
use std::path::Path;
use std::sync::LazyLock;

static EXTERNAL_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"name\s*=\s*"(.*)"\s*;"#).unwrap());

impl ExternalAddon {
    pub fn resolve_name(&mut self) -> anyhow::Result<()> {
        if self.name.is_some() {
            return Ok(());
        }

        let path = Path::new(&self.path);

        if !path.exists() {
            anyhow::bail!("Addon path does not exist: {:#?}", path);
        }

        if !path.is_dir() {
            anyhow::bail!("Addon path is not a directory: {:#?}", path);
        }

        let mod_path = path.join("mod.cpp");
        if mod_path.is_file() {
            let mod_name = try_find_name_in_file(&mod_path);

            if let Ok(Some(name)) = mod_name {
                self.name = Some(name);
                return Ok(());
            }
        }

        let meta_path = path.join("meta.cpp");
        if meta_path.is_file() {
            let meta_name = try_find_name_in_file(&meta_path);

            if let Ok(Some(name)) = meta_name {
                self.name = Some(name);
                return Ok(());
            }
        }

        log::debug!(
            "Could not find addon name in mod.cpp or meta.cpp for path {:#?}",
            path
        );

        Ok(())
    }
}

fn try_find_name_in_file(path: &Path) -> anyhow::Result<Option<String>> {
    let content = std::fs::read_to_string(path)?;
    Ok(EXTERNAL_NAME_REGEX
        .captures(&content)
        .map(|c| c[1].to_owned()))
}
