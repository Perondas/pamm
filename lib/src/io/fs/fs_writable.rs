use crate::models::keyed::Keyed;
use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::files::file_names::keyed_file::KeyedFile;
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

pub(crate) trait KnownFSWritable: FsWritable + FixedFile {
    fn write_to<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()>;
}

impl<T: FsWritable + FixedFile> KnownFSWritable for T {
    fn write_to<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let full_path = path.as_ref().join(Self::file_name());
        self.write_to_path(full_path)
            .context(anyhow!("writing {:?}", Self::file_name()))
    }
}

pub(crate) trait NamedFSWritable: FsWritable + KeyedFile {
    fn write_to_named<P: AsRef<Path>>(&self, path: P, identifier: &str) -> anyhow::Result<()>;
}

impl<T: FsWritable + KeyedFile> NamedFSWritable for T {
    fn write_to_named<P: AsRef<Path>>(&self, path: P, identifier: &str) -> anyhow::Result<()> {
        let full_path = path.as_ref().join(Self::file_name(identifier));
        self.write_to_path(full_path)
            .context(anyhow!("writing {:?}", identifier))
    }
}

pub(crate) trait IdentifiableFSWritable: NamedFSWritable + Keyed {
    fn write_to<P: AsRef<Path>>(&self, base_path: P) -> anyhow::Result<()>;
}

impl<T: NamedFSWritable + Keyed> IdentifiableFSWritable for T {
    fn write_to<P: AsRef<Path>>(&self, base_path: P) -> anyhow::Result<()> {
        self.write_to_named(base_path, self.get_key())
            .context(anyhow!("writing"))
    }
}
