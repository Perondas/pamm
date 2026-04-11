use crate::models::identifiable::Identifiable;
use crate::io::known_file::KnownFile;
use crate::io::named_file::NamedFile;
use crate::io::serialization::writable::Writable;
use anyhow::{Context, anyhow};
use std::path::Path;

pub(crate) trait FsWritable: Sized {
    fn write_to_path<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()>;
}

impl<T: Writable> FsWritable for T {
    fn write_to_path<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(&path)?;
        self.to_writer(&mut file)
            .context(anyhow!("writing {:?}", path.as_ref()))
    }
}

pub(crate) trait KnownFSWritable: FsWritable + KnownFile {
    fn write_to<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()>;
}

impl<T: FsWritable + KnownFile> KnownFSWritable for T {
    fn write_to<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let full_path = path.as_ref().join(Self::file_name());
        self.write_to_path(full_path)
            .context(anyhow!("writing {:?}", Self::file_name()))
    }
}

pub(crate) trait NamedFSWritable: FsWritable + NamedFile {
    fn write_to_named<P: AsRef<Path>>(&self, path: P, identifier: &str) -> anyhow::Result<()>;
}

impl<T: FsWritable + NamedFile> NamedFSWritable for T {
    fn write_to_named<P: AsRef<Path>>(&self, path: P, identifier: &str) -> anyhow::Result<()> {
        let full_path = path.as_ref().join(Self::get_file_name(identifier));
        self.write_to_path(full_path)
            .context(anyhow!("writing {:?}", identifier))
    }
}

pub(crate) trait IdentifiableFSWritable: NamedFSWritable + Identifiable {
    fn write_to<P: AsRef<Path>>(&self, base_path: P) -> anyhow::Result<()>;
}

impl<T: NamedFSWritable + Identifiable> IdentifiableFSWritable for T {
    fn write_to<P: AsRef<Path>>(&self, base_path: P) -> anyhow::Result<()> {
        self.write_to_named(base_path, self.get_identifier())
            .context(anyhow!("writing"))
    }
}
