use crate::io::net::http_client;
use crate::io::progress_reporting::copy_with_report::copy;
use crate::io::progress_reporting::download_reporter::DownloadReporter;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};
use url::Url;

pub(crate) async fn download_file(
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

    let resp = http_client::send(http_client::get(&file_url), &file_url).await?;

    let file = File::create(file_path).await?;
    let mut buffered_writer = BufWriter::new(file);
    let actual_len = copy(resp.bytes_stream(), &mut buffered_writer, reporter).await?;
    buffered_writer.flush().await?;

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
#[cfg(feature = "client")]
pub(crate) async fn download_file_unverified(
    file_path: &Path,
    file_url: Url,
) -> anyhow::Result<()> {
    use anyhow::Context;
    use tokio::fs;

    log::debug!("Downloading {} -> {:?} (unverified)", file_url, file_path);

    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .with_context(|| format!("Invalid download destination {:?}", file_path))?;
    let part_path = file_path.with_file_name(format!("{}.part", file_name));

    let resp = http_client::send(http_client::get(&file_url), &file_url).await?;

    let result = async {
        let file = File::create(&part_path).await?;
        let mut buffered_writer = BufWriter::new(file);
        let mut stream = resp.bytes_stream();
        while let Some(chunk) = futures::StreamExt::next(&mut stream).await {
            buffered_writer.write_all(&chunk?).await?;
        }
        buffered_writer.flush().await?;
        buffered_writer.into_inner().sync_all().await?;
        anyhow::Ok(())
    }
    .await;

    if let Err(e) = result {
        let _ = fs::remove_file(&part_path).await;
        return Err(e);
    }

    fs::rename(&part_path, file_path)
        .await
        .with_context(|| format!("Failed to move {:?} into place", part_path))?;

    Ok(())
}
