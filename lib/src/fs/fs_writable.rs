use crate::known_name::KnownName;
use crate::named::Named;
use crate::serialization::writable::Writable;
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

pub trait KnownFSWritable: FsWritable + KnownName {
    fn write_to_known<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()>;
}

impl<T: FsWritable + KnownName> KnownFSWritable for T {
    fn write_to_known<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let full_path = path.as_ref().join(Self::known_name());
        self.write_to_path(full_path)
    }
}

pub trait NamedFSWritable: FsWritable + Named {
    fn write_to_named<P: AsRef<Path>>(&self, path: P, identifier: &str) -> anyhow::Result<()>;
}

impl<T: FsWritable + Named> NamedFSWritable for T {
    fn write_to_named<P: AsRef<Path>>(&self, path: P, identifier: &str) -> anyhow::Result<()> {
        let full_path = path.as_ref().join(Self::get_name(identifier));
        self.write_to_path(full_path)
    }
}
