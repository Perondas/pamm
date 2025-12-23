use crate::io::name_consts::get_pack_addon_directory_name;
use crate::pack::pack_config::PackConfig;
use std::fs;
use std::path::Path;

impl PackConfig {
    pub fn init_blank_on_fs(&self, parent_dir: &Path) -> anyhow::Result<()> {
        if !parent_dir.is_dir() {
            anyhow::bail!("{} is not a directory", parent_dir.display());
        }

        let addon_dir_name = get_pack_addon_directory_name(&self.name);

        fs::create_dir(parent_dir.join(&addon_dir_name))?;

        Ok(())
    }
}
