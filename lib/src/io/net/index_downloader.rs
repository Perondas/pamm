use crate::index::index_node::IndexNode;
use crate::io::name_consts::{get_pack_addon_directory_name, INDEX_DIR_NAME};
use crate::pack::pack_config::PackConfig;
use crate::pack::pack_index::PackIndex;
use anyhow::Result;
use rayon::prelude::*;
use url::Url;
use crate::io::net::downloadable::NamedDownloadable;

impl PackConfig {
    pub fn download_index(&self, base_url: &Url) -> Result<PackIndex> {
        let addon_dir = base_url.join(&format!("{}/", get_pack_addon_directory_name(&self.name)))?;
        let index_dir = addon_dir.join(&format!("{}/", INDEX_DIR_NAME))?;

        let indexes = self
            .addons
            .par_iter()
            .map(|name| IndexNode::download_named(&index_dir, name))
            .collect::<Result<Vec<_>>>()?;

        Ok(PackIndex {
            addons: indexes,
            pack_name: self.name.clone(),
        })
    }
}
