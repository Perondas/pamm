use anyhow::Context;
use serde::Serialize;
use serde::de::DeserializeOwned;

// Human-readable JSON serialization

pub(crate) fn to_writer<E: Serialize, W: std::io::Write>(
    writer: &mut W,
    content: &E,
) -> anyhow::Result<()> {
    let mut buf_writer = std::io::BufWriter::new(writer);
    serde_json::to_writer_pretty(&mut buf_writer, content)?;
    std::io::Write::flush(&mut buf_writer).context("Failed to flush writer after serialization")
}

pub(crate) fn from_reader<D: DeserializeOwned, R: std::io::Read>(
    reader: &mut R,
) -> anyhow::Result<D> {
    let mut buf_reader = std::io::BufReader::new(reader);
    let res: Result<D, _> = serde_json::from_reader(&mut buf_reader);
    res.context("Failed to deserialize human-readable JSON data")
}
