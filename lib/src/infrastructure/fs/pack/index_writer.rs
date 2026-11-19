use crate::domain::index::checksum_index::ChecksumIndex;
use crate::domain::pack::pack_diff::PackDiff;
use crate::domain::pack::pack_index::PackIndex;
use crate::infrastructure::fs::files::fs_writable::{FixedFsWritable, SelfKeyedFSWritable};
use crate::infrastructure::fs::files::name_consts::INDEX_DIR_NAME;
use std::path::Path;

impl PackDiff {
    pub async fn write_checksum_index_to_fs(&self, base_path: &Path) -> anyhow::Result<()> {
        self.target_index
            .write_checksum_index_to_fs(base_path)
            .await
    }
}

impl PackIndex {
    pub async fn write_checksum_index_to_fs(&self, base_path: &Path) -> anyhow::Result<()> {
        let index_dir = base_path.join(&self.pack_name).join(INDEX_DIR_NAME);

        tokio::fs::create_dir_all(&index_dir).await?;

        let checksum_index = ChecksumIndex {
            checksums: self
                .addons
                .iter()
                .map(|node| (node.name.clone(), node.checksum.clone()))
                .collect(),
        };

        checksum_index.write_fixed(&index_dir).await
    }

    pub async fn write_full_index_to_fs(&self, base_path: &Path) -> anyhow::Result<()> {
        let index_dir = base_path.join(&self.pack_name).join(INDEX_DIR_NAME);

        tokio::fs::create_dir_all(&index_dir).await?;

        for addon in &self.addons {
            addon.write_to(&index_dir).await?;
        }

        self.write_checksum_index_to_fs(base_path).await
    }
}
