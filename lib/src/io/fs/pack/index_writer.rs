use crate::io::fs::fs_writable::NamedFSWritable;
use crate::io::name_consts::{INDEX_DIR_NAME, get_pack_addon_directory_name};
use crate::pack::pack_index::PackIndex;
use std::path::Path;

impl PackIndex {
    pub fn write_to_fs(&self, base_path: &Path) -> anyhow::Result<()> {
        let addon_dir = base_path.join(get_pack_addon_directory_name(&self.pack_name));
        let index_dir = addon_dir.join(INDEX_DIR_NAME);

        for addon in &self.addons {
            addon.write_to_named(&index_dir, &addon.name)?;
        }

        Ok(())
    }
}
