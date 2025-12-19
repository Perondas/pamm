pub(crate) trait Writable {
    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()>;
}
