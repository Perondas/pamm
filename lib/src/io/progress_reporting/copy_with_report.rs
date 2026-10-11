use crate::io::progress_reporting::download_reporter::DownloadReporter;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use log::debug;
use std::pin::pin;
use tokio::io::{AsyncWrite, AsyncWriteExt};

/// Drain `stream` into `writer`, reporting each chunk as it lands.
pub async fn copy<S, W>(
    stream: S,
    writer: &mut W,
    reporter: &impl DownloadReporter,
) -> anyhow::Result<u64>
where
    S: Stream<Item = reqwest::Result<Bytes>>,
    W: AsyncWrite + Unpin + ?Sized,
{
    let mut stream = pin!(stream);
    let mut total = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        writer.write_all(&chunk).await?;
        total += chunk.len() as u64;
        reporter.report_progress(chunk.len() as u64);
    }

    debug!(
        "Finished copying with progress report, total bytes copied: {}",
        total
    );
    Ok(total)
}
