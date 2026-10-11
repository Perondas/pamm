use crate::io::files::name_consts::INDEX_DIR_NAME;
use crate::io::net::downloadable::NamedDownloadable;
use crate::models::index::index_node::IndexNode;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_index::PackIndex;
use anyhow::Result;
use futures::{StreamExt, TryStreamExt, stream};
use url::Url;

/// How many index files are fetched at once.
const CONCURRENT_INDEX_DOWNLOADS: usize = 16;

impl PackConfig {
    pub async fn download_indexes(&self, base_url: &Url) -> Result<PackIndex> {
        log::info!(
            "Downloading indexes for pack '{}' from {}",
            self.name,
            base_url
        );

        let pack_dir = base_url.join(&format!("{}/", self.name))?;
        let index_dir = pack_dir.join(&format!("{}/", INDEX_DIR_NAME))?;

        log::debug!("Index directory URL: {}", index_dir);

        // Owned names: a closure over `&String` here trips rustc's `Send`
        // inference for the enclosing future (rust-lang/rust#110338).
        let index_dir = &index_dir;
        let indexes = stream::iter(self.addons.keys().cloned())
            .map(|addon| async move { IndexNode::download_named(index_dir, &addon).await })
            .buffered(CONCURRENT_INDEX_DOWNLOADS)
            .try_collect::<Vec<_>>()
            .await?;

        log::info!(
            "Downloaded {} index(es) for pack '{}'",
            indexes.len(),
            self.name
        );

        Ok(PackIndex {
            addons: indexes,
            pack_name: self.name.clone(),
        })
    }
}
