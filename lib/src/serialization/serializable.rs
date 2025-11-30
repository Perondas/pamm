use std::io::Read;

pub(crate) trait Readable: Sized {
    fn from_reader<R: Read>(reader: &mut R) -> anyhow::Result<Self>;
}

pub(crate) trait Writable {
    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()>;
}
