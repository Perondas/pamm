use crate::handle::repo_handle::RepoHandle;
use crate::io::files::file_paths::rel_path::RelPath;
use anyhow::{anyhow, ensure, Context};
use std::fs;

pub trait DeletePack {
    fn delete_pack(&mut self, pack_name: &str, delete_on_fs: bool) -> anyhow::Result<()>;
}

impl DeletePack for RepoHandle {
    fn delete_pack(&mut self, pack_name: &str, delete_on_fs: bool) -> anyhow::Result<()> {
        ensure!(
            self.repo_config.packs.contains(pack_name),
            "Pack '{}' not found in repo",
            pack_name
        );

        let pack_path = RelPath::from_name(pack_name);

        self.repo_config.packs.remove(pack_name);
        self.write(&pack_path, &self.repo_config)?;

        let pack_dir = self.repo_path.join(pack_name);
        if pack_dir.is_dir() {
            fs::remove_dir_all(&pack_dir).context(anyhow!("Failed to remove empty pack folder"))?;
        }

        Ok(())
    }
}
