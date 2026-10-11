use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::files::file_names::keyed_file::KeyedFile;
use crate::io::serialization::writable::Writable;
use crate::models::self_keyed::SelfKeyed;
use anyhow::{Context, anyhow};
use std::path::Path;

pub(crate) trait FsWritable: Sized {
    fn write_to_path<P: AsRef<Path> + Send + Sync>(
        &self,
        path: P,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

impl<T: Writable + Sync> FsWritable for T {
    async fn write_to_path<P: AsRef<Path> + Send + Sync>(&self, path: P) -> anyhow::Result<()> {
        // Serialize up front so the file is written in one go and never left
        // half-written by a serialization error.
        let mut buffer = Vec::new();
        self.to_writer(&mut buffer)
            .context(anyhow!("writing {:?}", path.as_ref()))?;
        tokio::fs::write(&path, buffer)
            .await
            .context(anyhow!("writing {:?}", path.as_ref()))
    }
}

pub(crate) trait FixedFsWritable: FsWritable + FixedFile {
    fn write_fixed<P: AsRef<Path> + Send + Sync>(
        &self,
        path: P,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

impl<T: FsWritable + FixedFile + Sync> FixedFsWritable for T {
    async fn write_fixed<P: AsRef<Path> + Send + Sync>(&self, path: P) -> anyhow::Result<()> {
        let full_path = path.as_ref().join(Self::file_name());
        self.write_to_path(full_path)
            .await
            .context(anyhow!("writing {:?}", Self::file_name()))
    }
}

pub(crate) trait KeyedFSWritable: FsWritable + KeyedFile {
    fn write_keyed<P: AsRef<Path> + Send + Sync>(
        &self,
        path: P,
        identifier: &str,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

impl<T: FsWritable + KeyedFile + Sync> KeyedFSWritable for T {
    async fn write_keyed<P: AsRef<Path> + Send + Sync>(
        &self,
        path: P,
        identifier: &str,
    ) -> anyhow::Result<()> {
        let full_path = path.as_ref().join(Self::file_name(identifier));
        self.write_to_path(full_path)
            .await
            .context(anyhow!("writing {:?}", identifier))
    }
}

pub(crate) trait SelfKeyedFSWritable: KeyedFSWritable + SelfKeyed {
    fn write_to<P: AsRef<Path> + Send + Sync>(
        &self,
        base_path: P,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

impl<T: KeyedFSWritable + SelfKeyed + Sync> SelfKeyedFSWritable for T {
    async fn write_to<P: AsRef<Path> + Send + Sync>(&self, base_path: P) -> anyhow::Result<()> {
        self.write_keyed(base_path, self.get_key())
            .await
            .context(anyhow!("writing"))
    }
}
