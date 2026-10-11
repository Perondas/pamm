use crate::io::files::file_paths::rel_path::RelPath;
use crate::io::files::name_consts::ADDONS_DIR_NAME;
use crate::io::net::byte_range_response::ByteRangeResponse;
use crate::io::net::download_file::download_file;
use crate::io::net::http_client;
use crate::io::progress_reporting::download_reporter::DownloadReporter;
use crate::models::index::index_node::{PBO_CHECKSUM_LEN, PBOPart};
use crate::models::index::node_diff::FileModification;
use crate::models::pack::pack_config::PackConfig;
use anyhow::{Context, Result, ensure};
use bi_fs_rs::pbo::handle::PBOHandle;
use std::collections::{HashMap, HashSet};
use std::io::SeekFrom;
use std::iter;
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufWriter};
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

    pub(crate) async fn patch_file(
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
                let local_parts = local_pbo_parts(file_path.to_path_buf()).await?;
                let mut local_pbo = File::open(file_path).await?;

                let temp_file_path = file_path.with_added_extension("pamm.temp");

                let mut temp_file = BufWriter::new(File::create(&temp_file_path).await?);

                let mut parts = self
                    .get_required_pbo_parts(
                        &file_url,
                        &new_order,
                        &required_checksums,
                        new_length,
                        new_blob_start,
                    )
                    .await?;

                let new_headers = parts
                    .next_part()
                    .await
                    .ok_or_else(|| anyhow::anyhow!("No response for PBO header"))??;

                temp_file.write_all(&new_headers).await?;
                temp_file.flush().await?;

                let written = temp_file.get_ref().metadata().await?.len();
                ensure!(
                    written == new_blob_start,
                    "Patched PBO blob start does not match expected blob start. Expected blob start: {}. Actual blob start: {}",
                    new_blob_start,
                    written
                );

                let required_checksums = required_checksums
                    .iter()
                    .map(Vec::as_slice)
                    .collect::<HashSet<_>>();

                for part in new_order {
                    if required_checksums.contains(part.checksum.as_slice()) {
                        let part_data = parts
                            .next_part()
                            .await
                            .ok_or_else(|| anyhow::anyhow!("No response for PBO part"))??;
                        ensure!(
                            part_data.len() == part.length as usize,
                            "Received PBO part length does not match expected length. Expected length: {}. Actual length: {}",
                            part.length,
                            part_data.len()
                        );
                        temp_file.write_all(&part_data).await?;
                    } else {
                        let &(offset, length) = local_parts
                            .get(&part.name)
                            .with_context(|| format!("File not found in PBO: {}", part.name))?;
                        let mut data = vec![0; length as usize];
                        local_pbo.seek(SeekFrom::Start(offset)).await?;
                        local_pbo.read_exact(&mut data).await?;
                        ensure!(
                            data.len() == part.length as usize,
                            "Cached PBO part length does not match expected length. Expected length: {}. Actual length: {}",
                            part.length,
                            data.len()
                        );
                        temp_file.write_all(&data).await?;
                    }
                }

                let checksum_data = parts
                    .next_part()
                    .await
                    .ok_or_else(|| anyhow::anyhow!("No response for PBO checksum"))??;

                ensure!(
                    checksum_data.len() as u64 == PBO_CHECKSUM_LEN,
                    "Received PBO checksum length does not match expected length. Expected length: {}. Actual length: {}",
                    PBO_CHECKSUM_LEN,
                    checksum_data.len()
                );

                temp_file.write_all(&checksum_data).await?;

                if parts.next_part().await.is_some() {
                    return Err(anyhow::anyhow!("Received more entries than expected"));
                }

                temp_file.flush().await?;
                let written = temp_file.get_ref().metadata().await?.len();
                if written != new_length {
                    return Err(anyhow::anyhow!(
                        "Patched PBO length does not match expected length. Expected length: {}. Actual length: {}",
                        new_length,
                        written
                    ));
                }

                drop(local_pbo);
                drop(temp_file);
                fs::rename(temp_file_path, file_path).await?;

                log::debug!("PBO patch applied successfully to {:?}", file_path);
                Ok(())
            }
            FileModification::Generic { new_length } => {
                log::debug!(
                    "Downloading generic file replacement ({} bytes)",
                    new_length
                );
                download_file(file_path, file_url, new_length, &self.reporter).await
            }
        }
    }

    pub(crate) async fn create_file(
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
        download_file(file_path, file_url, expected_len, &self.reporter).await
    }

    async fn get_required_pbo_parts(
        &self,
        url: &Url,
        new_order: &[PBOPart],
        required_checksums: &[Vec<u8>],
        new_length: u64,
        blob_start: u64,
    ) -> Result<ByteRangeResponse<P>> {
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
        let request = http_client::get(url).header("Range", format!("bytes={}", ranges_str));
        let resp = http_client::send(request, url)
            .await
            .context(format!("Failed to fetch range for {}", url))?;

        ByteRangeResponse::from_response(resp, self.reporter.clone())
    }
}

/// Where each file's content sits inside the local PBO: name to (absolute
/// offset, length). The first entry wins for duplicate names, as in
/// [`PBOHandle::get_file_content`].
async fn local_pbo_parts(path: PathBuf) -> Result<HashMap<String, (u64, u64)>> {
    tokio::task::spawn_blocking(move || {
        let handle = PBOHandle::open_file(&path)?;

        let mut parts = HashMap::with_capacity(handle.files.len());
        let mut offset = handle.blob_start;
        for file in &handle.files {
            parts
                .entry(file.filename.to_string())
                .or_insert((offset, file.size as u64));
            offset += file.size as u64;
        }

        Ok(parts)
    })
    .await?
}
