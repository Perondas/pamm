use crate::handle::reading::get_pack::GetPack;
use crate::handle::repo_handle::RepoHandle;
use crate::io::fs::fs_readable::NamedFSReadable;
use crate::io::name_consts::{INDEX_DIR_NAME, get_pack_addon_directory_name};
use crate::models::index::index_node::IndexNode;
use crate::models::pack::pack_index::PackIndex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub trait GetPackIndex {
    fn get_pack_index(&self, pack_name: &str) -> anyhow::Result<PackIndex>;
}

impl GetPackIndex for RepoHandle {
    fn get_pack_index(&self, pack_name: &str) -> anyhow::Result<PackIndex> {
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
}
