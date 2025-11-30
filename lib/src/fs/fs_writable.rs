use crate::serialization::serializable::Writable;
use std::path::Path;

pub(crate) trait FsWritable: Sized {
    fn write_to_path<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()>;
}

impl<T: Writable> FsWritable for T {
    fn write_to_path<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(path)?;
        self.to_writer(&mut file)
    }
}
