use crate::io::known_file::KnownFile;
use crate::io::named_file::NamedFile;
use crate::io::serialization::readable::Readable;
use std::path::Path;

pub trait FsReadable: Sized {
    fn read_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Option<Self>>;
}

impl<T: Readable> FsReadable for T {
    fn read_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Option<Self>> {
        if !path.as_ref().exists() {
            return Ok(None);
        }

        let mut file = std::fs::File::open(path)?;
        Self::from_reader(&mut file).map(Some)
    }
}

pub trait KnownFSReadable: FsReadable + KnownFile {
    fn read_from_known<P: AsRef<Path>>(path: P) -> anyhow::Result<Option<Self>>;
}

impl<T: FsReadable + KnownFile> KnownFSReadable for T {
    fn read_from_known<P: AsRef<Path>>(path: P) -> anyhow::Result<Option<Self>> {
        let full_path = path.as_ref().join(Self::file_name());
        Self::read_from_path(full_path)
    }
}

pub trait NamedFSReadable: FsReadable + NamedFile {
    fn read_from_named<P: AsRef<Path>>(path: P, identifier: &str) -> anyhow::Result<Option<Self>>;
}

impl<T: FsReadable + NamedFile> NamedFSReadable for T {
    fn read_from_named<P: AsRef<Path>>(path: P, identifier: &str) -> anyhow::Result<Option<Self>> {
        let full_path = path.as_ref().join(Self::get_file_name(identifier));
        Self::read_from_path(full_path)
    }
}
