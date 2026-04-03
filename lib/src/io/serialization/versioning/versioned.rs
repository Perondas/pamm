use crate::io::known_file::KnownFile;
use crate::io::named_file::NamedFile;
use crate::io::serialization::readable::Readable;
use crate::io::serialization::writable::Writable;
use std::io::{Read, Write};

pub(crate) trait Versioned: Sized + Clone {
    type WrapperType: Readable + Writable + VersionedWrapper<Self>;
}

pub(crate) trait VersionedWrapper<T: Sized>: Sized {
    fn get(self) -> T;
    fn wrap(value: T) -> Self;
}

impl<T: Versioned> KnownFile for T
where
    T::WrapperType: KnownFile,
{
    fn file_name() -> &'static str {
        <T::WrapperType as KnownFile>::file_name()
    }
}

impl<T: Versioned> NamedFile for T
where
    T::WrapperType: NamedFile,
{
    fn get_file_name(ident: &str) -> String {
        <T::WrapperType as NamedFile>::get_file_name(ident)
    }
}

impl<T: Versioned> Readable for T {
    fn from_reader<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        let versioned = T::WrapperType::from_reader(reader)?;
        Ok(versioned.get())
    }
}

impl<T: Versioned> Writable for T {
    fn to_writer<W: Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        let versioned = T::WrapperType::wrap(self.to_owned());
        versioned.to_writer(writer)
    }
}
