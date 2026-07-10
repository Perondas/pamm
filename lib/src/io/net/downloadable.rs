use crate::io::known_file::KnownFile;
use crate::io::named_file::NamedFile;
use crate::io::serialization::readable::Readable;
use anyhow::Context;
use log::debug;
use url::Url;

pub trait Downloadable: Sized {
    fn download(url: &Url) -> anyhow::Result<Self>;
}

impl<T: Readable> Downloadable for T {
    fn download(url: &Url) -> anyhow::Result<Self> {
        Self::from_reader(
            &mut ureq::get(url.to_string())
                .call()
                .context(format!("Failed to download: {}", url))?
                .body_mut()
                .as_reader(),
        )
        .context(format!("Failed to deserialize {}", url))
    }
}

pub trait KnownDownloadable: Downloadable + KnownFile {
    fn download_known(url: &Url) -> anyhow::Result<Self>;
}

impl<T: Downloadable + KnownFile> KnownDownloadable for T {
    fn download_known(url: &Url) -> anyhow::Result<Self> {
        let mut full_url = url.clone();

        full_url
            .path_segments_mut()
            .unwrap_or_else(|_| panic!("Bad base url: {:?}", url))
            .pop_if_empty()
            .push(Self::file_name());
        debug!("Downloading known file from URL: {}", full_url);
        Self::download(&full_url)
    }
}

pub trait NamedDownloadable: Downloadable + NamedFile {
    fn download_named(url: &Url, identifier: &str) -> anyhow::Result<Self>;
}

impl<T: Downloadable + NamedFile> NamedDownloadable for T {
    fn download_named(url: &Url, identifier: &str) -> anyhow::Result<Self> {
        let mut full_url = url.clone();

        full_url
            .path_segments_mut()
            .unwrap_or_else(|_| panic!("Bad base url: {:?}", url))
            .pop_if_empty()
            .extend(
                Self::get_rel_path(identifier)
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().into_owned()),
            );
        debug!("Downloading named file from URL: {}", full_url);
        Self::download(&full_url)
    }
}
