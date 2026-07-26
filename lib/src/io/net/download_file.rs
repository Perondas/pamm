use crate::io::progress_reporting::copy_with_report::copy;
use crate::io::progress_reporting::download_reporter::DownloadReporter;
use anyhow::Context;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use url::Url;

pub(crate) fn download_file(
    file_path: &Path,
    file_url: Url,
    expected_len: u64,
    reporter: &impl DownloadReporter,
) -> anyhow::Result<()> {
    log::debug!(
        "Downloading {} -> {:?} (expected {} bytes)",
        file_url,
        file_path,
        expected_len
    );

    let resp = ureq::get(file_url.to_string()).call()?;
    let body = resp.into_body();

    let file = File::create(file_path)?;
    let mut buffered_writer = BufWriter::new(file);
    let actual_len = copy(&mut body.into_reader(), &mut buffered_writer, reporter)?;

    if actual_len != expected_len {
        anyhow::bail!(
            "Downloaded file length {} does not match expected length {} for file {}",
            actual_len,
            expected_len,
            file_url
        );
    }

    log::debug!("Downloaded {} ({} bytes)", file_url, actual_len);
    Ok(())
}

/// Download a file whose length is not known up front (nothing indexes it).
/// Writes to a `<name>.part` sibling first and renames on success, so the
/// destination only ever holds a complete file.
pub(crate) fn download_file_unverified(file_path: &Path, file_url: Url) -> anyhow::Result<()> {
    log::debug!("Downloading {} -> {:?} (unverified)", file_url, file_path);

    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .with_context(|| format!("Invalid download destination {:?}", file_path))?;
    let part_path = file_path.with_file_name(format!("{}.part", file_name));

    let resp = ureq::get(file_url.to_string()).call()?;
    let body = resp.into_body();

    let result = (|| -> anyhow::Result<()> {
        let file = File::create(&part_path)?;
        let mut buffered_writer = BufWriter::new(file);
        std::io::copy(&mut body.into_reader(), &mut buffered_writer)?;
        buffered_writer.into_inner()?.sync_all()?;
        Ok(())
    })();

    if let Err(e) = result {
        let _ = fs::remove_file(&part_path);
        return Err(e);
    }

    fs::rename(&part_path, file_path)
        .with_context(|| format!("Failed to move {:?} into place", part_path))?;

    Ok(())
}
