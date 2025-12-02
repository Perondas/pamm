use crate::fs::fs_writable::{FsWritable, KnownFSWritable};
use crate::name_consts::*;
use crate::pack::config::pack_config::PackConfig;
use crate::pack::manifest::pack_manifest::PackManifest;
use std::fs;
use std::path::Path;

pub fn init_pack_on_fs(pack_config: &PackConfig, parent_dir: &Path) -> anyhow::Result<()> {
    if !parent_dir.is_dir() {
        anyhow::bail!("{} is not a directory", parent_dir.display());
    }

    let base_path = parent_dir.join(&pack_config.name);

    fs::create_dir(&base_path)?;

    fs::create_dir(base_path.join(REQUIRED_DIR_NAME))?;
    fs::create_dir(base_path.join(OPTIONAL_DIR_NAME))?;

    pack_config.write_to_path(&base_path)?;

    PackManifest::default().write_to_known(&base_path)?;

    Ok(())
}
