use std::fs;
use crate::pack::pack_manifest::{PackConfig, PackManifest};
use anyhow::Result;
use crate::consts::*;

impl PackConfig {
    pub fn init_on_disk(&self) -> Result<()> {
        let base_path = std::env::current_dir()?.join(&self.name);
        
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