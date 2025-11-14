use crate::consts::*;
use crate::pack::pack_config::PackConfig;
use crate::pack::pack_manifest::PackManifest;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

impl PackConfig {
    pub fn init_on_disk(&self, base: &PathBuf) -> Result<()> {
        let base_path = base.join(&self.name);

        fs::create_dir(&base_path)?;

        fs::create_dir(base_path.join(REQUIRED_DIR_NAME))?;
        fs::create_dir(base_path.join(OPTIONAL_DIR_NAME))?;
        fs::create_dir(base_path.join(STATE_DIR_NAME))?;

        let config_path = base_path.join(CONFIG_FILE_NAME);
        let config_file = fs::File::create(config_path)?;
        serde_json::to_writer_pretty(config_file, &self)?;

        let manifest = PackManifest::default();
        let manifest_path = base_path.join(MANIFEST_FILE_NAME);
        let manifest_file = fs::File::create(manifest_path)?;
        serde_cbor::to_writer(manifest_file, &manifest)?;

        Ok(())
    }
}
