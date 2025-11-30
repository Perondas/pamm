use crate::pack::pack_manifest::PackManifest;
use crate::serialization::readable::Readable;
use crate::serialization::serializers::bin_serializer;
use crate::serialization::writable::Writable;
use std::io::Read;

impl Readable for PackManifest {
    fn from_reader<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        bin_serializer::from_reader(reader)
    }
}

impl Writable for PackManifest {
    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        bin_serializer::to_writer(writer, self)
    }
}
