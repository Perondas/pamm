use crate::index::index_node::IndexNode;
use crate::index::node_diff::{NodeDiff, NodeModification};
use crate::io::fs::fs_deletable::NamedFsDeletable;
use crate::io::fs::fs_writable::IdentifiableFSWritable;
use crate::io::name_consts::{INDEX_DIR_NAME, get_pack_addon_directory_name};
use crate::pack::pack_diff::PackDiff;
use crate::pack::pack_index::PackIndex;
use std::path::Path;

impl PackIndex {
    pub fn write_to_fs(&self, base_path: &Path) -> anyhow::Result<()> {
        let addon_dir = base_path.join(get_pack_addon_directory_name(&self.pack_name));
        let index_dir = addon_dir.join(INDEX_DIR_NAME);

        for addon in &self.addons {
            addon.write_to(&index_dir)?;
        }

        Ok(())
    }
}

impl PackDiff {
    pub fn write_index_to_fs(&self, base_path: &Path, new_index: &PackIndex) -> anyhow::Result<()> {
        let addon_dir = base_path.join(get_pack_addon_directory_name(&new_index.pack_name));
        let index_dir = addon_dir.join(INDEX_DIR_NAME);

        for diff in &self.0 {
            match diff {
                NodeDiff::Created(IndexNode { name, .. })
                | NodeDiff::Modified(NodeModification { name, .. }) => {
                    if let Some(new_node) = new_index.addons.iter().find(|node| &node.name == name)
                    {
                        new_node.write_to(&index_dir)?;
                    }
                }
                NodeDiff::Deleted(name) => {
                    IndexNode::delete_named(&index_dir, name)?;
                }
                NodeDiff::None => (),
            }
        }

        Ok(())
    }
}
