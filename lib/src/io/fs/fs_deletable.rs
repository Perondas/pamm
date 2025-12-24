use crate::io::known_file::KnownFile;
use crate::io::named::NamedFile;
use std::path::Path;

pub trait KnownFsDeletable: Sized + KnownFile {
    fn delete_known<P: AsRef<Path>>(path: P) -> anyhow::Result<()>;
}

impl<T: KnownFile + Sized> KnownFsDeletable for T {
    fn delete_known<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let full_path = path.as_ref().join(Self::file_name());
        if full_path.exists() {
            std::fs::remove_file(full_path)?;
        }
        Ok(())
    }
}

pub trait NamedFsDeletable: Sized + NamedFile {
    fn delete_named<P: AsRef<Path>>(path: P, identifier: &str) -> anyhow::Result<()>;
}

impl<T: NamedFile + Sized> NamedFsDeletable for T {
    fn delete_named<P: AsRef<Path>>(path: P, identifier: &str) -> anyhow::Result<()> {
        let full_path = path.as_ref().join(Self::get_file_name(identifier));
        if full_path.exists() {
            std::fs::remove_file(full_path)?;
        }
        Ok(())
    }
}
