use crate::handle::repo_handle::RepoHandle;
use crate::io::name_consts::get_pack_addon_directory_name;
use anyhow::{Context, anyhow, ensure};

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

        self.repo_config.packs.remove(pack_name);
        self.write(&self.repo_config)?;

        std::fs::remove_file(self.repo_path.join(format!("{}.json", pack_name)))?;

        if delete_on_fs {
            let addons_path = get_pack_addon_directory_name(pack_name);
            let full_addons_path = self.repo_path.join(addons_path);

            if full_addons_path.exists() {
                std::fs::remove_dir_all(full_addons_path)
                    .context(anyhow!("Failed to delete addons directory"))?;
            }
        }

        Ok(())
    }
}
