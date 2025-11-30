use crate::pack::pack_config::PackConfig;
use crate::serialization::serializable::{Readable, Writable};
use std::io::Read;

impl Readable for PackConfig {
    fn from_reader<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        super::json_serialization::from_reader(reader)
    }
}

impl Writable for PackConfig {
    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        super::json_serialization::to_writer(writer, self)
    }
}
