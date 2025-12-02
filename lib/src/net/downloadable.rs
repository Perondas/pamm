use crate::known_name::KnownName;
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
