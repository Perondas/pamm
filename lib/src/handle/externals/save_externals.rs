use crate::handle::externals::external_addon::ExternalAddon;
use crate::handle::repo_handle::RepoHandle;
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use std::sync::LazyLock;

static EXTERNAL_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"name\s*=\s*"(.*)"\s*;"#).unwrap());

impl RepoHandle {
    pub fn save_externals(
        &self,
        pack_name: &str,
        externals: &[ExternalAddon],
    ) -> anyhow::Result<()> {
        let (_, mut settings) = self.get_pack_with_settings(pack_name)?;

        settings.external_addons = externals
            .iter()
            .map(|e| e.to_owned())
            .map(|mut e| {
                if e.name.is_none() {
                    e.name = get_external_addons_name(&e.path)?;
                }
                Ok(e)
            })
            .collect::<anyhow::Result<HashSet<_>>>()?;

        self.write_named(&settings, pack_name)?;

        Ok(())
    }
}

fn get_external_addons_name(path: &str) -> anyhow::Result<Option<String>> {
    let path = Path::new(path);

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
            return Ok(Some(name));
        }
    }

    let meta_path = path.join("meta.cpp");
    if meta_path.is_file() {
        let meta_name = try_find_name_in_file(&meta_path);

        if let Ok(Some(name)) = meta_name {
            return Ok(Some(name));
        }
    }

    log::debug!(
        "Could not find addon name in mod.cpp or meta.cpp for path {:#?}",
        path
    );

    Ok(None)
}

fn try_find_name_in_file(path: &Path) -> anyhow::Result<Option<String>> {
    let content = std::fs::read_to_string(path)?;
    Ok(EXTERNAL_NAME_REGEX
        .captures(&content)
        .map(|c| c[1].to_owned()))
}
