use anyhow::Context;
use serde::Serialize;
use serde::de::DeserializeOwned;

// Human-readable JSON serialization

pub(in crate::serialization) fn to_writer<E: Serialize, W: std::io::Write>(
    writer: &mut W,
    content: &E,
) -> anyhow::Result<()> {
    serde_json::to_writer_pretty(writer, content)?;
    Ok(())
}

pub(in crate::serialization) fn from_reader<D: DeserializeOwned, R: std::io::Read>(
    reader: &mut R,
) -> anyhow::Result<D> {
    let res: Result<D, _> = serde_json::from_reader(reader);
    res.context("Failed to deserialize human-readable JSON data")
}
