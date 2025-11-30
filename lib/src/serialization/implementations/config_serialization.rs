use crate::pack::pack_config::PackConfig;
use crate::serialization::readable::Readable;
use crate::serialization::serializers::hr_serializer;
use crate::serialization::writable::Writable;
use std::io::Read;

impl Readable for PackConfig {
    fn from_reader<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        hr_serializer::from_reader(reader)
    }
}

impl Writable for PackConfig {
    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        hr_serializer::to_writer(writer, self)
    }
}
