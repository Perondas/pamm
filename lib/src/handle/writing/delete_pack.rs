use crate::handle::repo_handle::RepoHandle;
use crate::io::name_consts::{pack_addons_rel, pack_config_rel, pack_indexes_rel, pack_settings_rel};
use anyhow::{Context, anyhow, ensure};
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

        self.repo_config.packs.remove(pack_name);
        self.write(&self.repo_config)?;

        let config_path = self.repo_path.join(pack_config_rel(pack_name));
        if config_path.exists() {
            fs::remove_file(config_path).context(anyhow!("Failed to delete pack config file"))?;
        }

        let settings_path = self.repo_path.join(pack_settings_rel(pack_name));
        if settings_path.exists() {
            fs::remove_file(settings_path)
                .context(anyhow!("Failed to delete pack settings file"))?;
        }

        let indexes_path = self.repo_path.join(pack_indexes_rel(pack_name));
        if indexes_path.exists() {
            fs::remove_dir_all(indexes_path)
                .context(anyhow!("Failed to delete indexes directory"))?;
        }

        if delete_on_fs {
            let full_addons_path = self.repo_path.join(pack_addons_rel(pack_name));

            if full_addons_path.exists() {
                fs::remove_dir_all(full_addons_path)
                    .context(anyhow!("Failed to delete addons directory"))?;
            }
        }

        // The pack folder may now be empty (or the addons dir was intentionally
        // kept when delete_on_fs is false) — only remove it once nothing is left.
        let pack_dir = self.repo_path.join(pack_name);
        if pack_dir.is_dir() && fs::read_dir(&pack_dir)?.next().is_none() {
            fs::remove_dir(&pack_dir).context(anyhow!("Failed to remove empty pack folder"))?;
        }

        Ok(())
    }
}
