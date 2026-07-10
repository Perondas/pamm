use crate::io::fs::fs_writable::{IdentifiableFSWritable, KnownFSWritable};
use crate::io::name_consts::pack_indexes_rel;
use crate::models::index::checksum_index::ChecksumIndex;
use crate::models::pack::pack_diff::PackDiff;
use crate::models::pack::pack_index::PackIndex;
use std::path::Path;

impl PackDiff {
    pub fn write_checksum_index_to_fs(&self, base_path: &Path) -> anyhow::Result<()> {
        self.target_index.write_checksum_index_to_fs(base_path)
    }
}

impl PackIndex {
    pub fn write_checksum_index_to_fs(&self, base_path: &Path) -> anyhow::Result<()> {
        let index_dir = base_path.join(pack_indexes_rel(&self.pack_name));

        std::fs::create_dir_all(&index_dir)?;

        let checksum_index = ChecksumIndex {
            checksums: self
                .addons
                .iter()
                .map(|node| (node.name.clone(), node.checksum.clone()))
                .collect(),
        };

        checksum_index.write_to(&index_dir)
    }

    pub fn write_full_index_to_fs(&self, base_path: &Path) -> anyhow::Result<()> {
        let index_dir = base_path.join(pack_indexes_rel(&self.pack_name));

        std::fs::create_dir_all(&index_dir)?;

        for addon in &self.addons {
            addon.write_to(&index_dir)?;
        }

        self.write_checksum_index_to_fs(base_path)
    }
}
