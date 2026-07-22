use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::io::files::file_paths::keyed_path::KeyedFilePath;
use crate::io::net::downloadable::KnownDownloadable;
use anyhow::Context;
use url::Url;

impl ClientRepoHandle {
    pub(super) fn download_keyed<T: KnownDownloadable + KeyedFilePath>(
        &self,
        key: &str,
    ) -> anyhow::Result<T> {
        let url = T::file_path(key).with_base_url(self.get_remote_url());

        T::download_known(&url).with_context(|| {
            format!(
                "Failed to download named file '{}' from repository",
                T::file_name()
            )
        })
    }

    pub(super) fn get_remote_url(&self) -> &Url {
        self.user_settings().get_remote()
    }
}
