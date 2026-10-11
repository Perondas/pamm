use crate::io::fs::fs_readable::KnownFSReadable;
use crate::models::pack::pack_config::PackConfig;
use anyhow::{Context, anyhow, bail};
use std::path::Path;

/// Removes a pack from disk. Does nothing if pack does not exist. Errors if pack is malformed
pub async fn remove_pack(parent_dir: impl AsRef<Path>, pack_name: &str) -> anyhow::Result<()> {
    let pack_path = parent_dir.as_ref().join(pack_name);

    let Ok(metadata) = tokio::fs::metadata(&pack_path).await else {
        // Nothing to do
        return Ok(());
    };

    if !metadata.is_dir() {
        bail!(
            "Pack path {} exists but is not a directory",
            pack_path.display()
        );
    }

    // Check that we were about to delete a proper pack
    let _ = PackConfig::read_from_known(&pack_path)
        .await
        .context(anyhow!(
            "Failed to read pack config from: {}. Won't delete malformed pack",
            pack_path.display()
        ))?;

    tokio::fs::remove_dir_all(&pack_path)
        .await
        .with_context(|| format!("Failed to remove pack: {}", pack_path.display()))
}
