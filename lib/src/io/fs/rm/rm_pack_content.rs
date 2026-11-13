use crate::io::files::name_consts::{ADDONS_DIR_NAME, INDEX_DIR_NAME};
use crate::io::fs::fs_readable::KnownFSReadable;
use crate::models::pack::pack_config::PackConfig;
use anyhow::{Context, anyhow, ensure};
use std::path::Path;

/// Clears the packs content, fails if pack does not exist
pub async fn remove_pack_content(
    parent_dir: impl AsRef<Path>,
    pack_name: &str,
) -> anyhow::Result<()> {
    let pack_path = parent_dir.as_ref().join(pack_name);

    ensure!(
        pack_path.is_dir(),
        "Pack path {} exists but is not a directory",
        pack_path.display()
    );

    // Ensure well-formed
    let _ = PackConfig::read_from_known(&pack_path)
        .await
        .context(anyhow!(
            "Failed to read pack config from: {}. Won't delete malformed pack",
            pack_path.display()
        ))?;

    let addon_dir = pack_path.join(ADDONS_DIR_NAME);

    // Way easier than cleaning out the folder :)
    tokio::fs::remove_dir_all(&addon_dir)
        .await
        .with_context(|| format!("Failed to remove addons: {}", addon_dir.display()))?;

    tokio::fs::create_dir_all(&addon_dir)
        .await
        .with_context(|| format!("Failed to recreate addons dir: {}", addon_dir.display()))?;

    let index_dir = pack_path.join(INDEX_DIR_NAME);

    tokio::fs::remove_dir_all(&index_dir)
        .await
        .with_context(|| format!("Failed to remove index: {}", index_dir.display()))?;

    tokio::fs::create_dir_all(&index_dir)
        .await
        .with_context(|| format!("Failed to recreate index dir: {}", index_dir.display()))?;

    Ok(())
}
