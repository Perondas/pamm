use anyhow::Context;
use serde::Serialize;
use serde::de::DeserializeOwned;

// Binary serialization using bincode

pub(crate) fn to_writer<E: Serialize, W: std::io::Write>(
    writer: &mut W,
    content: &E,
) -> anyhow::Result<()> {
    let mut buf_writer = std::io::BufWriter::new(writer);
    bincode::serde::encode_into_std_write(content, &mut buf_writer, bincode::config::standard())?;
    std::io::Write::flush(&mut buf_writer)?;
    Ok(())
}

pub(crate) fn from_reader<D: DeserializeOwned, R: std::io::Read>(
    reader: &mut R,
) -> anyhow::Result<D> {
    let mut buf_reader = std::io::BufReader::new(reader);
    let res: Result<D, _> =
        bincode::serde::decode_from_std_read(&mut buf_reader, bincode::config::standard());
    res.context("Failed to deserialize binary data")
}
