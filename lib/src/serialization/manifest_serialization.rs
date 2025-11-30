use crate::pack::pack_manifest::PackManifest;
use crate::serialization::serializable::{Readable, Writable};
use std::io::Read;

impl Readable for PackManifest {
    fn from_reader<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        super::bin_serialization::from_reader(reader)
    }
}

impl Writable for PackManifest {
    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        super::bin_serialization::to_writer(writer, self)
    }
}
