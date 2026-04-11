use crate::io::known_file::KnownFile;
use crate::io::named_file::NamedFile;
use crate::io::serialization::readable::Readable;
use anyhow::{Context, anyhow};
use std::path::Path;

pub(crate) trait FsReadable: Sized {
    fn read_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self>;
}

impl<T: Readable> FsReadable for T {
    fn read_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut file =
            std::fs::File::open(&path).context(anyhow!("opening {:?}", path.as_ref()))?;
        Self::from_reader(&mut file)
    }
}

pub(crate) trait KnownFSReadable: FsReadable + KnownFile {
    fn read_from_known<P: AsRef<Path>>(path: P) -> anyhow::Result<Self>;
}

impl<T: FsReadable + KnownFile> KnownFSReadable for T {
    fn read_from_known<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let full_path = path.as_ref().join(Self::file_name());
        Self::read_from_path(full_path).context(anyhow!("reading {:?}", Self::file_name()))
    }
}

pub(crate) trait NamedFSReadable: FsReadable + NamedFile {
    fn read_from_named<P: AsRef<Path>>(path: P, identifier: &str) -> anyhow::Result<Self>;
}

impl<T: FsReadable + NamedFile> NamedFSReadable for T {
    fn read_from_named<P: AsRef<Path>>(path: P, identifier: &str) -> anyhow::Result<Self> {
        let full_path = path.as_ref().join(Self::get_file_name(identifier));
        Self::read_from_path(full_path).context(anyhow!("reading {:?}", identifier))
    }
}
