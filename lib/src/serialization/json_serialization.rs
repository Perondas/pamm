use serde::de::DeserializeOwned;
use serde::Serialize;

pub(super) fn to_writer<E: Serialize, W: std::io::Write>(
    writer: &mut W,
    content: &E,
) -> anyhow::Result<()> {
    serde_json::to_writer(writer, content)?;
    Ok(())
}

pub(super) fn from_reader<D: DeserializeOwned, R: std::io::Read>(reader: &mut R) -> anyhow::Result<D> {
    let res: Result<D, _> = serde_json::from_reader(reader);
    Ok(res?)
}
