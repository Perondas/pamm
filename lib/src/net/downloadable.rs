use crate::known_name::KnownName;
use crate::named::Named;
use crate::serialization::readable::Readable;
use anyhow::Context;
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

pub trait KnownDownloadable: Downloadable + KnownName {
    fn download_known(url: &Url) -> anyhow::Result<Self>;
}

impl<T: Downloadable + KnownName> KnownDownloadable for T {
    fn download_known(url: &Url) -> anyhow::Result<Self> {
        let full_url = url.join(Self::known_name())?;
        Self::download(&full_url)
    }
}

pub trait NamedDownloadable: Downloadable + Named {
    fn download_named(url: &Url, identifier: &str) -> anyhow::Result<Self>;
}

impl<T: Downloadable + Named> NamedDownloadable for T {
    fn download_named(url: &Url, identifier: &str) -> anyhow::Result<Self> {
        let full_url = url.join(&Self::get_name(identifier))?;
        Self::download(&full_url)
    }
}
