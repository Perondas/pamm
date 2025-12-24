use crate::index::index_node::IndexNode;
use crate::io::fs::fs_readable::NamedFSReadable;
use crate::io::name_consts::{INDEX_DIR_NAME, get_pack_addon_directory_name};
use crate::pack::pack_config::PackConfig;
use crate::pack::pack_index::PackIndex;
use anyhow::Result;
use std::path::Path;

impl PackConfig {
    pub fn read_index_from_fs(&self, base_path: &Path) -> Result<PackIndex> {
        let addon_dir = base_path.join(get_pack_addon_directory_name(&self.name));
        let index_dir = addon_dir.join(INDEX_DIR_NAME);

        let indexes = self
            .addons
            .keys()
            .filter_map(|name| IndexNode::read_from_named(&index_dir, name).transpose())
            .collect::<Result<Vec<_>>>()?;

        Ok(PackIndex {
            addons: indexes,
            pack_name: self.name.clone(),
        })
    }
}
