use crate::io::fs::fs_deletable::NamedFsDeletable;
use crate::io::fs::fs_writable::{IdentifiableFSWritable, KnownFSWritable};
use crate::io::name_consts::{INDEX_DIR_NAME, get_pack_addon_directory_name};
use crate::models::index::checksum_index::ChecksumIndex;
use crate::models::index::index_node::IndexNode;
use crate::models::index::node_diff::{NodeDiff, NodeModification};
use crate::models::pack::pack_diff::PackDiff;
use std::path::Path;

impl PackDiff {
    pub fn write_index_to_fs(&self, base_path: &Path) -> anyhow::Result<()> {
        let addon_dir = base_path.join(get_pack_addon_directory_name(self.get_pack_name()));
        let index_dir = addon_dir.join(INDEX_DIR_NAME);

        for diff in &self.addon_diffs {
            match diff {
                NodeDiff::Created(IndexNode { name, .. })
                | NodeDiff::Modified(NodeModification { name, .. }) => {
                    if let Some(new_node) = self
                        .target_index
                        .addons
                        .iter()
                        .find(|node| &node.name == name)
                    {
                        new_node.write_to(&index_dir)?;
                    }
                }
                NodeDiff::Deleted { name, .. } => {
                    IndexNode::delete_named(&index_dir, name)?;
                }
                NodeDiff::None(_) => (),
            }
        }

        self.write_checksum_index_to_fs(base_path)?;

        Ok(())
    }

    pub fn write_checksum_index_to_fs(&self, base_path: &Path) -> anyhow::Result<()> {
        let index_dir = base_path
            .join(get_pack_addon_directory_name(self.get_pack_name()))
            .join(INDEX_DIR_NAME);

        std::fs::create_dir_all(&index_dir)?;

        let checksum_index = ChecksumIndex {
            checksums: self
                .target_index
                .addons
                .iter()
                .map(|node| (node.name.clone(), node.checksum.clone()))
                .collect(),
        };

        checksum_index.write_to(&index_dir)?;

        Ok(())
    }
}
