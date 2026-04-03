use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::fs_readable::{KnownFSReadable, NamedFSReadable};
use crate::io::fs::util::clean_path::clean_path;
use crate::io::name_consts::{INDEX_DIR_NAME, get_pack_addon_directory_name};
use crate::models::index::index_node::IndexNode;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_index::PackIndex;
use crate::models::pack::pack_user_settings::PackUserSettings;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;
use anyhow::{Context, anyhow, ensure};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

impl RepoHandle {
    pub fn get_config(&self) -> &RepoConfig {
        &self.repo_config
    }

    pub fn get_pack(&self, pack_name: &str) -> anyhow::Result<PackConfig> {
        ensure!(
            self.repo_config.packs.contains(pack_name),
            "Pack '{}' not found in repo",
            pack_name
        );

        self.read_named::<PackConfig>(pack_name).context(anyhow!(
            "Failed to read pack config for {} in {:#?}",
            pack_name,
            self.repo_path
        ))
    }

    pub fn get_pack_with_settings(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<(PackConfig, PackUserSettings)> {
        let pack_config = self.get_pack(pack_name)?;

        let pack_user_settings = self.read_named(pack_name).context(anyhow!(
            "Failed to read settings for {} in {:#?}",
            pack_name,
            self.repo_path
        ))?;

        Ok((pack_config, pack_user_settings))
    }

    pub fn get_repo_user_settings(&self) -> anyhow::Result<&RepoUserSettings> {
        self.repo_user_settings
            .as_ref()
            .ok_or(anyhow!("Repo user settings not found"))
    }

    pub fn get_pack_index(&self, pack_name: &str) -> anyhow::Result<PackIndex> {
        let pack_config = self.get_pack(pack_name)?;

        let addon_dir = self
            .repo_path
            .join(get_pack_addon_directory_name(pack_name));
        let index_dir = addon_dir.join(INDEX_DIR_NAME);

        let indexes = pack_config
            .addons
            .par_iter()
            .map(|addon| IndexNode::read_from_named(&index_dir, addon.0))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(PackIndex {
            addons: indexes,
            pack_name: pack_config.name.clone(),
        })
    }

    pub fn get_addon_paths(&self, pack_name: &str) -> anyhow::Result<Vec<String>> {
        log::debug!("Resolving addon paths for pack '{}'", pack_name);

        let addons = self
            .resolve_addons(pack_name)?
            .iter()
            .chain(&self.resolve_optionals_paths(pack_name)?)
            .map(|p| {
                p.canonicalize()
                    .with_context(|| format!("Failed to canonicalize {:#?}", p))
            })
            .map(|p| p.map(clean_path))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let externals = self
            .get_external_addon_paths(pack_name)
            .context(anyhow!("Failed to read external addons"))?;

        Ok([addons, externals].concat())
    }

    #[allow(dead_code)]
    pub(super) fn read<T: KnownFSReadable>(&self) -> anyhow::Result<T> {
        T::read_from_known(&self.repo_path)
    }

    pub(super) fn read_named<T: NamedFSReadable>(&self, identifier: &str) -> anyhow::Result<T> {
        T::read_from_named(&self.repo_path, identifier)
    }
}
