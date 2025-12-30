use crate::identifiable::Identifiable;
use crate::io::known_file::KnownFile;
use crate::io::named_file::NamedFile;
use crate::io::serialization::writable::Writable;
use std::path::Path;

pub trait FsWritable: Sized {
    fn write_to_path<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()>;
}

impl<T: Writable> FsWritable for T {
    fn write_to_path<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(path)?;
        self.to_writer(&mut file)
    }
}

pub trait KnownFSWritable: FsWritable + KnownFile {
    fn write_to<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()>;
}

impl<T: FsWritable + KnownFile> KnownFSWritable for T {
    fn write_to<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let full_path = path.as_ref().join(Self::file_name());
        self.write_to_path(full_path)
    }
}

pub trait NamedFSWritable: FsWritable + NamedFile {
    fn write_to_named<P: AsRef<Path>>(&self, path: P, identifier: &str) -> anyhow::Result<()>;
}

impl<T: FsWritable + NamedFile> NamedFSWritable for T {
    fn write_to_named<P: AsRef<Path>>(&self, path: P, identifier: &str) -> anyhow::Result<()> {
        let full_path = path.as_ref().join(Self::get_file_name(identifier));
        self.write_to_path(full_path)
    }
}

pub trait IdentifiableFSWritable: NamedFSWritable + Identifiable {
    fn write_to<P: AsRef<Path>>(&self, base_path: P) -> anyhow::Result<()>;
}

impl<T: NamedFSWritable + Identifiable> IdentifiableFSWritable for T {
    fn write_to<P: AsRef<Path>>(&self, base_path: P) -> anyhow::Result<()> {
        self.write_to_named(base_path, self.get_identifier())
    }
}
