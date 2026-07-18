use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::files::file_names::keyed_file::KeyedFile;
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

pub trait KnownDownloadable: Downloadable + FixedFile {
    fn download_known(url: &Url) -> anyhow::Result<Self>;
}

impl<T: Downloadable + FixedFile> KnownDownloadable for T {
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

pub trait NamedDownloadable: Downloadable + KeyedFile {
    fn download_named(url: &Url, identifier: &str) -> anyhow::Result<Self>;
}

impl<T: Downloadable + KeyedFile> NamedDownloadable for T {
    fn download_named(url: &Url, identifier: &str) -> anyhow::Result<Self> {
        let mut full_url = url.clone();

        full_url
            .path_segments_mut()
            .unwrap_or_else(|_| panic!("Bad base url: {:?}", url))
            .pop_if_empty()
            .push(&Self::file_name(identifier));
        debug!("Downloading named file from URL: {}", full_url);
        Self::download(&full_url)
    }
}
