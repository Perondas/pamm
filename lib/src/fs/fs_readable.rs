use crate::serialization::serializable::Readable;
use std::path::Path;

pub(crate) trait FsReadable: Sized {
    fn read_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self>;
}

impl<T: Readable> FsReadable for T {
    fn read_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut file = std::fs::File::open(path)?;
        Self::from_reader(&mut file)
    }
}
