use crate::known_name::KnownName;
use crate::serialization::readable::Readable;
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

pub trait KnownFSReadable: FsReadable + KnownName {
    fn read_from_known<P: AsRef<Path>>(path: P) -> anyhow::Result<Option<Self>>;
}

impl<T: FsReadable + KnownName> KnownFSReadable for T {
    fn read_from_known<P: AsRef<Path>>(path: P) -> anyhow::Result<Option<Self>> {
        let full_path = path.as_ref().join(Self::known_name());
        Self::read_from_path(full_path)
    }
}
