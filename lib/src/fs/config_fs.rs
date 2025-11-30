use crate::consts::*;
use crate::pack::pack_config::PackConfig;
use crate::pack::pack_manifest::PackManifest;
use std::fs;
use std::path::Path;
use crate::fs::fs_readable::FsReadable;

impl PackConfig {
    pub fn init_on_disk(&self, parent_dir: &Path) -> anyhow::Result<()> {
        if !parent_dir.is_dir() {
            anyhow::bail!("{} is not a directory", parent_dir.display());
        }

        let base_path = parent_dir.join(&self.name);

        fs::create_dir(&base_path)?;

        fs::create_dir(base_path.join(REQUIRED_DIR_NAME))?;
        fs::create_dir(base_path.join(OPTIONAL_DIR_NAME))?;
        fs::create_dir(base_path.join(STATE_DIR_NAME))?;

        let config_path = base_path.join(CONFIG_FILE_NAME);
        let config_file = fs::File::create(config_path)?;
        serde_json::to_writer_pretty(config_file, &self)?;

        let manifest = PackManifest::default();
        manifest.write_to_fs(&base_path)?;

        Ok(())
    }

    pub fn read(base_path: &Path) -> anyhow::Result<Self> {
        let path = base_path.join(CONFIG_FILE_NAME);
        if path.exists() {
            PackConfig::read_from_path(&path)
        } else {
            anyhow::bail!("no config file found")
        }
    }
}
