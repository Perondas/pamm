use crate::pack::part_diff::PBOModification;
use std::fs::File;
use std::iter;
use std::path::Path;
use url::Url;

pub fn download_file(destination_path: &Path, url: Url) -> anyhow::Result<()> {
    let resp = ureq::get(&url.to_string()).call()?;
    let body = resp.into_body();

    let mut file = File::create(destination_path)?;
    std::io::copy(&mut body.into_reader(), &mut file)?;
    Ok(())
}

pub fn patch_pbo_file(
    destination_path: &Path,
    url: Url,
    pbo_modification: PBOModification,
) -> anyhow::Result<()> {
    let request_builder = ureq::get(&url.to_string());

    let PBOModification {
        name,
        new_order,
        required_parts,
        target_checksum,
        blob_offset,
    } = pbo_modification;

    // We always get the entire header
    // TODO: can we only get part of it? Is that worth it?
    let pbo_header_range = (0_u64, pbo_modification.blob_offset - 1);
    let modified_ranges = new_order
        .iter()
        .filter(|p| required_parts.contains(&p.checksum))
        .map(|p| {
            let start = p.start_offset + blob_offset;
            (start, start + p.length as u64)
        });

    let ranges = iter::once(pbo_header_range)
        .chain(modified_ranges)
        .map(|(from, to)| format!("{}-{}", from, to))
        .collect::<Vec<_>>()
        .join(", ");
    let resp = request_builder
        .header("Range", format!("bytes={}", ranges))
        .call()?;

    let (p,b)= resp.into_parts();

    todo!()

    /* let ranges = ranges
        .iter()
        .map(|(from, to)| format!("{}-{}", from, to))
        .collect::<Vec<_>>()
        .join(", ");
    request_builder.header("Range", format!("bytes={}", ranges))*/
}
