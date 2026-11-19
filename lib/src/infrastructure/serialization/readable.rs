use std::io::Read;

pub(crate) trait Readable: Sized {
    fn from_reader<R: Read>(reader: &mut R) -> anyhow::Result<Self>;
}
