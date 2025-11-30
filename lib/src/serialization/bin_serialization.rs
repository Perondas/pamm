use serde::de::DeserializeOwned;
use serde::Serialize;

pub(super) fn to_writer<E: Serialize, W: std::io::Write>(
    writer: &mut W,
    content: &E,
) -> anyhow::Result<()> {
    bincode::serde::encode_into_std_write(content, writer, bincode::config::standard())?;
    Ok(())
}

pub(super) fn from_reader<D: DeserializeOwned, R: std::io::Read>(
    reader: &mut R,
) -> anyhow::Result<D> {
    let res: Result<D, _> =
        bincode::serde::decode_from_std_read(reader, bincode::config::standard());
    Ok(res?)
}
