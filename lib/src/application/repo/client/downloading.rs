use crate::application::repo::client::ClientRepoHandle;
use crate::infrastructure::filesystem::files::file_paths::keyed_path::KeyedFilePath;
use crate::infrastructure::http::downloadable::KnownDownloadable;
use anyhow::Context;
use url::Url;

impl ClientRepoHandle {
    pub(in crate::application) async fn download_keyed<T: KnownDownloadable + KeyedFilePath>(
        &self,
        key: &str,
    ) -> anyhow::Result<T> {
        let url = T::file_path(key).with_base_url(self.get_remote_url());

        T::download_known(&url).await.with_context(|| {
            format!(
                "Failed to download named file '{}' from repository",
                T::file_name()
            )
        })
    }

    pub(in crate::application) fn get_remote_url(&self) -> &Url {
        self.user_settings().get_remote()
    }
}
