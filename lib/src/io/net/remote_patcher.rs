use crate::io::files::name_consts::ADDONS_DIR_NAME;
use crate::io::net::byte_range_response::{ByteRangeResponse, IntoByteRangeResponse};
use crate::io::net::download_file::download_file;
use crate::io::progress_reporting::download_reporter::DownloadReporter;
use crate::io::files::file_paths::rel_path::RelPath;
use crate::models::index::index_node::{PBO_CHECKSUM_LEN, PBOPart};
use crate::models::index::node_diff::FileModification;
use crate::models::pack::pack_config::PackConfig;
use anyhow::{Context, Result, ensure};
use bi_fs_rs::pbo::handle::PBOHandle;
use fs::File;
use std::collections::HashSet;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{fs, iter};
use ureq::BodyReader;
use url::Url;

pub(crate) struct RemotePatcher<P: DownloadReporter> {
    addon_dir_url: Url,
    reporter: P,
}

impl PackConfig {
    pub(crate) fn remote_patcher<P: DownloadReporter>(
        &self,
        base_url: &Url,
        reporter: P,
    ) -> RemotePatcher<P> {
        let addon_dir_url = base_url
            .join(&format!("{}/{}/", self.name, ADDONS_DIR_NAME))
            .expect("Failed to construct addon dir URL");

        RemotePatcher::new(addon_dir_url, reporter)
    }
}

impl<P: DownloadReporter> RemotePatcher<P> {
    pub(crate) fn new(addon_dir_url: Url, reporter: P) -> Self {
        Self {
            addon_dir_url,
            reporter,
        }
    }

    pub(crate) fn patch_file(
        &self,
        rel_path: &RelPath,
        file_path: &Path,
        modification: FileModification,
    ) -> Result<()> {
        let file_url = rel_path.with_base_url(&self.addon_dir_url);
        log::debug!("Patching file {:?} from {}", file_path, file_url);

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

                let mut temp_file = BufWriter::new(File::create(&temp_file_path)?);

                let mut parts = self.get_required_pbo_parts(
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
                temp_file.flush()?;

                ensure!(
                    temp_file.get_ref().metadata()?.len() == new_blob_start,
                    "Patched PBO blob start does not match expected blob start. Expected blob start: {}. Actual blob start: {}",
                    new_blob_start,
                    temp_file.get_ref().metadata()?.len()
                );

                let required_checksums = required_checksums
                    .iter()
                    .map(Vec::as_slice)
                    .collect::<HashSet<_>>();

                for part in new_order {
                    if required_checksums.contains(part.checksum.as_slice()) {
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

                ensure!(
                    checksum_data.len() as u64 == PBO_CHECKSUM_LEN,
                    "Received PBO checksum length does not match expected length. Expected length: {}. Actual length: {}",
                    PBO_CHECKSUM_LEN,
                    checksum_data.len()
                );

                temp_file.write_all(&checksum_data)?;

                if parts.next().is_some() {
                    return Err(anyhow::anyhow!("Received more entries than expected"));
                }

                temp_file.flush()?;
                if temp_file.get_ref().metadata()?.len() != new_length {
                    return Err(anyhow::anyhow!(
                        "Patched PBO length does not match expected length. Expected length: {}. Actual length: {}",
                        new_length,
                        temp_file.get_ref().metadata()?.len()
                    ));
                }

                drop(pbo_handle);
                drop(temp_file);
                fs::rename(temp_file_path, file_path)?;

                log::debug!("PBO patch applied successfully to {:?}", file_path);
                Ok(())
            }
            FileModification::Generic { new_length } => {
                log::debug!(
                    "Downloading generic file replacement ({} bytes)",
                    new_length
                );
                download_file(file_path, file_url, new_length, &self.reporter)
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
        log::debug!(
            "Creating file {:?} from {} ({} bytes)",
            file_path,
            file_url,
            expected_len
        );
        download_file(file_path, file_url, expected_len, &self.reporter)
    }

    fn get_required_pbo_parts(
        &self,
        url: &Url,
        new_order: &[PBOPart],
        required_checksums: &[Vec<u8>],
        new_length: u64,
        blob_start: u64,
    ) -> Result<ByteRangeResponse<BodyReader<'static>, P>> {
        let request_builder = ureq::get(url.to_string());

        // Ranges are inclusive

        // We always get the entire header + the padding bit
        // TODO: can we only get part of it? Is that worth it?
        let pbo_header_range = (0_u64, blob_start - 1);
        let pbo_checksum_range = (new_length - PBO_CHECKSUM_LEN, new_length - 1);

        let required_checksums = required_checksums
            .iter()
            .map(Vec::as_slice)
            .collect::<HashSet<_>>();

        let modified_ranges = new_order
            .iter()
            .filter(|p| required_checksums.contains(p.checksum.as_slice()))
            .map(|p| {
                let start = p.start_offset + blob_start;
                (start, start + p.length as u64 - 1)
            });

        let ranges = iter::once(pbo_header_range)
            .chain(modified_ranges)
            .chain(iter::once(pbo_checksum_range))
            .collect::<Vec<_>>();

        let ranges_str = ranges
            .iter()
            .map(|(from, to)| format!("{}-{}", from, to))
            .collect::<Vec<_>>()
            .join(", ");
        let resp = request_builder
            .header("Range", format!("bytes={}", ranges_str))
            .call()
            .context(format!("Failed to fetch range for {}", url))?;

        let responses = resp.into_byte_range_response(self.reporter.clone())?;

        Ok(responses)
    }
}
