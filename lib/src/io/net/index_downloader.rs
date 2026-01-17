use crate::index::index_node::IndexNode;
use crate::io::name_consts::{INDEX_DIR_NAME, get_pack_addon_directory_name};
use crate::io::net::downloadable::NamedDownloadable;
use crate::pack::pack_config::PackConfig;
use crate::pack::pack_index::PackIndex;
use anyhow::Result;
use rayon::prelude::*;
use url::Url;

impl PackConfig {
    pub fn download_indexes(&self, base_url: &Url) -> Result<PackIndex> {
        let addon_dir =
            base_url.join(&format!("{}/", get_pack_addon_directory_name(&self.name)))?;
        let index_dir = addon_dir.join(&format!("{}/", INDEX_DIR_NAME))?;

        let indexes = self
            .addons
            .par_iter()
            .map(|addon| IndexNode::download_named(&index_dir, addon.0))
            .collect::<Result<Vec<_>>>()?;

        Ok(PackIndex {
            addons: indexes,
            pack_name: self.name.clone(),
        })
    }
}
