use crate::io::fs::fs_writable::{IdentifiableFSWritable, NamedFSWritable};
use crate::io::name_consts::{INDEX_DIR_NAME, get_pack_addon_directory_name};
use crate::pack::pack_config::PackConfig;
use crate::pack::pack_user_settings::PackUserSettings;
use std::fs;
use std::path::Path;

impl PackConfig {
    pub fn init_blank_on_fs(&self, parent_dir: &Path) -> anyhow::Result<()> {
        if !parent_dir.is_dir() {
            anyhow::bail!("{} is not a directory", parent_dir.display());
        }

        let addon_dir_name = get_pack_addon_directory_name(&self.name);

        fs::create_dir(parent_dir.join(&addon_dir_name))?;

        let index_dir = parent_dir.join(&addon_dir_name).join(INDEX_DIR_NAME);

        fs::create_dir(&index_dir)?;

        let settings = PackUserSettings::default();

        settings.write_to_named(&index_dir, &self.name)?;

        self.write_to(parent_dir)?;

        Ok(())
    }
}
