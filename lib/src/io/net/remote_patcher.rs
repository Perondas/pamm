use crate::index::index_node::PBOPart;
use crate::index::node_diff::FileModification;
use crate::io::name_consts::get_pack_addon_directory_name;
use crate::io::net::byte_range_response::{ByteRangeResponse, IntoByteRangeResponse};
use crate::io::net::download_file::download_file;
use crate::io::rel_path::RelPath;
use crate::pack::pack_config::PackConfig;
use anyhow::{ensure, Context, Result};
use bi_fs_rs::pbo::handle::PBOHandle;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, iter};
use ureq::BodyReader;
use url::Url;

pub struct RemotePatcher {
    addon_dir_url: Url,
}

impl PackConfig {
    pub fn remote_patcher(&self, base_url: &Url) -> RemotePatcher {
        let addon_dir_url = base_url
            .join(&format!("{}/", get_pack_addon_directory_name(&self.name)))
            .expect("Failed to construct addon dir URL");

        RemotePatcher::new(addon_dir_url)
    }
}

impl RemotePatcher {
    pub(crate) fn new(addon_dir_url: Url) -> Self {
        Self { addon_dir_url }
    }

    pub(crate) fn patch_file(
        &self,
        rel_path: &RelPath,
        file_path: &Path,
        modification: FileModification,
    ) -> Result<()> {
        let file_url = rel_path.with_base_url(&self.addon_dir_url);

        match modification {
            FileModification::PBO {
                new_length,
                new_order,
                required_checksums,
                new_blob_start,
                ..
            } => {
                let mut pbo_handle = PBOHandle::open_file(file_path)?;

                let temp_file_path = file_path.with_added_extension("pamm.temp");

                let mut temp_file = File::create(&temp_file_path)?;

                let mut parts = get_required_pbo_parts(
                    &file_url,
                    &new_order,
                    &required_checksums,
                    new_length,
                    new_blob_start,
                )?;

                let new_headers = parts
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("No response for PBO header"))??;

                temp_file.write_all(&new_headers)?;

                for part in new_order {
                    if required_checksums.contains(&part.checksum) {
                        let part_data = parts
                            .next()
                            .ok_or_else(|| anyhow::anyhow!("No response for PBO part"))??;
                        ensure!(
                            part_data.len() == part.length as usize,
                            "Received PBO part length does not match expected length. Expected length: {}. Actual length: {}",
                            part.length,
                            part_data.len()
                        );
                        temp_file.write_all(&part_data)?;
                    } else {
                        let data = pbo_handle.get_file_content(&part.name)?;
                        ensure!(
                            data.len() == part.length as usize,
                            "Cached PBO part length does not match expected length. Expected length: {}. Actual length: {}",
                            part.length,
                            data.len()
                        );
                        temp_file.write_all(&data)?;
                    }
                }

                let checksum_data = parts
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("No response for PBO checksum"))??;
                temp_file.write_all(&checksum_data)?;

                if parts.next().is_some() {
                    return Err(anyhow::anyhow!("Received more entries than expected"));
                }

                if temp_file.metadata()?.len() != new_length {
                    return Err(anyhow::anyhow!(
                        "Patched PBO length does not match expected length. Expected length: {}. Actual length: {}",
                        new_length,
                        temp_file.metadata()?.len()
                    ));
                }

                drop(pbo_handle);
                drop(temp_file);
                fs::rename(temp_file_path, file_path)?;

                Ok(())
            }
            FileModification::Generic { new_length } => {
                download_file(file_path, file_url, new_length)
            }
        }
    }

    pub(crate) fn create_file(
        &self,
        rel_path: &RelPath,
        file_path: &Path,
        expected_len: u64,
    ) -> Result<()> {
        let file_url = rel_path.with_base_url(&self.addon_dir_url);

        download_file(file_path, file_url, expected_len)
    }
}

fn get_required_pbo_parts(
    url: &Url,
    new_order: &[PBOPart],
    required_checksums: &[Vec<u8>],
    new_length: u64,
    blob_start: u64,
) -> Result<ByteRangeResponse<BodyReader<'static>>> {
    let request_builder = ureq::get(url.to_string());

    // Ranges are inclusive

    // We always get the entire header + the padding bit
    // TODO: can we only get part of it? Is that worth it?
    let pbo_header_range = (0_u64, blob_start - 1);
    let pbo_checksum_rage = (new_length - 19, new_length);

    let modified_ranges = new_order
        .iter()
        .filter(|p| required_checksums.contains(&p.checksum))
        .map(|p| {
            let start = p.start_offset + blob_start;
            (start, start + p.length as u64)
        });

    let ranges = iter::once(pbo_header_range)
        .chain(modified_ranges)
        .chain(iter::once(pbo_checksum_rage))
        .collect::<Vec<_>>();

    let ranges_str = ranges
        .iter()
        .map(|(from, to)| format!("{}-{}", from, to - 1))
        .collect::<Vec<_>>()
        .join(", ");
    let resp = request_builder
        .header("Range", format!("bytes={}", ranges_str))
        .call()
        .context(format!("Failed to fetch range for {}", url))?;

    let responses = resp.into_byte_range_response()?;

    Ok(responses)
}
