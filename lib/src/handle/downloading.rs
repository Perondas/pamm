use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::io::net::downloadable::KnownDownloadable;
use crate::io::files::file_paths::rel_path::RelPath;
use anyhow::Context;
use url::Url;

impl ClientRepoHandle {
    pub(super) fn download_known<T: KnownDownloadable>(
        &self,
        rel_path: &RelPath,
    ) -> anyhow::Result<T> {
        let url = rel_path
            .push(T::file_name())
            .with_base_url(self.get_remote_url());

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
