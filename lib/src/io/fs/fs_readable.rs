use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::serialization::readable::Readable;
use anyhow::{Context, anyhow};
use std::path::Path;

pub(crate) trait FsReadable: Sized {
    fn read_from_path<P: AsRef<Path> + Send + Sync>(
        path: P,
    ) -> impl Future<Output = anyhow::Result<Self>> + Send;
}

impl<T: Readable> FsReadable for T {
    async fn read_from_path<P: AsRef<Path> + Send + Sync>(path: P) -> anyhow::Result<Self> {
        let bytes = tokio::fs::read(&path)
            .await
            .context(anyhow!("opening {:?}", path.as_ref()))?;
        Self::from_reader(&mut bytes.as_slice())
    }
}

pub(crate) trait KnownFSReadable: FsReadable + FixedFile {
    fn read_from_known<P: AsRef<Path> + Send + Sync>(
        path: P,
    ) -> impl Future<Output = anyhow::Result<Self>> + Send;
}

impl<T: FsReadable + FixedFile> KnownFSReadable for T {
    async fn read_from_known<P: AsRef<Path> + Send + Sync>(path: P) -> anyhow::Result<Self> {
        let full_path = path.as_ref().join(Self::file_name());
        Self::read_from_path(full_path)
            .await
            .context(anyhow!("reading {:?}", Self::file_name()))
    }
}
