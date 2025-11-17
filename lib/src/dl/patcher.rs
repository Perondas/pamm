use crate::dl::multipart::{ByteRangeResponse, IntoByteRangeResponse};
use crate::pack::part_diff::{GenericFileModification, PBOModification};
use bi_fs_rs::pbo::handle::PBOHandle;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, iter, mem};
use ureq::BodyReader;
use url::Url;

pub fn download_file(destination_path: &Path, url: Url) -> anyhow::Result<()> {
    let resp = ureq::get(&url.to_string()).call()?;
    let body = resp.into_body();

    let mut file = File::create(destination_path)?;
    std::io::copy(&mut body.into_reader(), &mut file)?;
    Ok(())
}

pub fn patch_generic_file(
    destination_path: &Path,
    url: Url,
    _: &GenericFileModification,
) -> anyhow::Result<()> {
    download_file(destination_path, url)
}

pub fn patch_pbo_file(
    existing_file: &Path,
    url: Url,
    pbo_modification: &PBOModification,
) -> anyhow::Result<()> {
    let mut pbo_handle = PBOHandle::open_file(existing_file)?;

    let temp_file_path = existing_file.with_added_extension("pamm.temp");

    let mut temp_file = File::create(&temp_file_path)?;

    let mut parts = get_required_pbo_parts(&url, &pbo_modification)?;

    let PBOModification {
        new_order,
        required_parts,
        ..
    } = pbo_modification;

    let new_headers = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("No response for PBO header"))??;

    temp_file.write_all(&new_headers)?;

    for part in new_order {
        if required_parts.contains(&part.checksum) {
            let part_data = parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("No response for PBO part"))??;
            temp_file.write_all(&part_data)?;
        } else {
            let data = pbo_handle.get_file_content(&part.name)?;
            temp_file.write_all(&data)?;
        }
    }

    if parts.next().is_some() {
        return Err(anyhow::anyhow!("Received more parts than expected"));
    }

    mem::drop(pbo_handle);
    mem::drop(temp_file);
    fs::rename(temp_file_path, existing_file)?;

    Ok(())
}

fn get_required_pbo_parts(
    url: &Url,
    pbo_modification: &PBOModification,
) -> anyhow::Result<ByteRangeResponse<BodyReader<'static>>> {
    let PBOModification {
        new_order,
        required_parts,
        blob_offset,
        ..
    } = pbo_modification;

    let request_builder = ureq::get(&url.to_string());

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
        .collect::<Vec<_>>();

    let ranges_str = ranges
        .iter()
        .map(|(from, to)| format!("{}-{}", from, to))
        .collect::<Vec<_>>()
        .join(", ");
    let resp = request_builder
        .header("Range", format!("bytes={}", ranges_str))
        .call()?;

    let responses = resp.into_byte_range_response()?;

    Ok(responses)
}
