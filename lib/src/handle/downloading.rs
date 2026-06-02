use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::io::net::downloadable::NamedDownloadable;
use anyhow::Context;
use url::Url;

impl ClientRepoHandle {
    pub(super) fn download_named<T: NamedDownloadable>(
        &self,
        identifier: &str,
    ) -> anyhow::Result<T> {
        T::download_named(self.get_remote_url(), identifier).with_context(|| {
            format!(
                "Failed to download named file '{}' with identifier '{}' from repository",
                T::get_file_name(identifier),
                identifier
            )
        })
    }

    pub(super) fn get_remote_url(&self) -> &Url {
        self.user_settings().get_remote()
    }
}
