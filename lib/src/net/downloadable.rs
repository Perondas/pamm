use crate::serialization::readable::Readable;
use url::Url;

pub trait Downloadable: Sized {
    fn download(url: &Url) -> anyhow::Result<Self>;
}

impl<T: Readable> Downloadable for T {
    fn download(url: &Url) -> anyhow::Result<Self> {
        Self::from_reader(&mut ureq::get(url.to_string()).call()?.body_mut().as_reader())
    }
}
