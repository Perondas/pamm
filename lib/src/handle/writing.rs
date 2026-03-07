use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::fs_writable::{KnownFSWritable, NamedFSWritable};
use crate::io::name_consts::get_pack_addon_directory_name;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_diff::PackDiff;
use crate::models::repo::repo_config::RepoConfig;
use anyhow::{Context, anyhow, ensure};

impl RepoHandle {
    pub fn add_pack(&mut self, pack_config: &PackConfig) -> anyhow::Result<()> {
        ensure!(
            !self.repo_config.packs.contains(&pack_config.name),
            "Pack '{}' already exists in repo",
            pack_config.name
        );

        self.repo_config.packs.insert(pack_config.name.clone());
        self.write(&self.repo_config)?;

        self.write_named(pack_config, &pack_config.name)?;

        pack_config.init_blank_on_fs(&self.repo_path)?;

        Ok(())
    }

    pub fn delete_pack(&mut self, pack_name: &str, delete_on_fs: bool) -> anyhow::Result<()> {
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

    pub fn update_pack(&self, pack_config: &PackConfig) -> anyhow::Result<()> {
        ensure!(
            self.repo_config.packs.contains(&pack_config.name),
            "Pack '{}' not found in repo",
            pack_config.name
        );

        self.write_named(pack_config, &pack_config.name)
    }

    pub fn update_repo_config(&mut self, repo_config: RepoConfig) -> anyhow::Result<()> {
        self.write(&repo_config)?;
        self.repo_config = repo_config;
        Ok(())
    }

    pub fn apply_diff(&self, diff: &PackDiff) -> anyhow::Result<()> {
        let pack_config = self.get_pack(diff.get_pack_name())?;

        diff.write_index_to_fs(&self.repo_path)?;

        let config = pack_config.update_addons(diff);

        self.update_pack(&config)
    }

    pub(super) fn write_named<T: NamedFSWritable>(
        &self,
        value: &T,
        identifier: &str,
    ) -> anyhow::Result<()> {
        value.write_to_named(&self.repo_path, identifier)
    }

    pub(super) fn write<T: KnownFSWritable>(&self, value: &T) -> anyhow::Result<()> {
        value.write_to(&self.repo_path)
    }
}
