use crate::known_name::KnownName;
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
