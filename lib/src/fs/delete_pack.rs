use crate::manifest::pack_manifest::PackManifest;
use crate::name_consts::get_pack_addon_directory_name;
use crate::named::Named;
use crate::pack::pack_config::PackConfig;
use anyhow::{Context, Result, anyhow};
use std::path::Path;

pub fn delete_pack(base_path: &Path, pack_name: &str) -> Result<()> {
    let config_path = PackConfig::get_name(pack_name);

    let full_path = base_path.join(config_path);

    if full_path.exists() {
        std::fs::remove_file(full_path).context(anyhow!("Failed to delete config file"))?;
    }

    let addons_path = get_pack_addon_directory_name(pack_name);
    let full_addons_path = base_path.join(addons_path);

    if full_addons_path.exists() {
        std::fs::remove_dir_all(full_addons_path)
            .context(anyhow!("Failed to delete addons directory"))?;
    }

    let manifest_path = PackManifest::get_name(pack_name);
    let full_manifest_path = base_path.join(manifest_path);
    if full_manifest_path.exists() {
        std::fs::remove_file(full_manifest_path)
            .context(anyhow!("Failed to delete manifest file"))?;
    }

    Ok(())
}
