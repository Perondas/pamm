use crate::io::files::file_names::fixed_file::FixedFile;
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

pub(crate) trait KnownFSReadable: FsReadable + FixedFile {
    fn read_from_known<P: AsRef<Path>>(path: P) -> anyhow::Result<Self>;
}

impl<T: FsReadable + FixedFile> KnownFSReadable for T {
    fn read_from_known<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let full_path = path.as_ref().join(Self::file_name());
        Self::read_from_path(full_path).context(anyhow!("reading {:?}", Self::file_name()))
    }
}

