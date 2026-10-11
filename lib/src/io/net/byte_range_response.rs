use crate::io::progress_reporting::download_reporter::DownloadReporter;
use anyhow::Context;
use anyhow::anyhow;
use futures::TryStreamExt;
use regex::Regex;
use reqwest::Response;
use reqwest::header::CONTENT_TYPE;
use std::pin::Pin;
use std::sync::LazyLock;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader};
use tokio_util::io::StreamReader;

static CONTENT_RANGE_HEADER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^content-range: bytes (\d+)-(\d+)/(\d+)\r\n$").unwrap());

type BodyReader = Pin<Box<dyn AsyncRead + Send>>;

/// The parts of a `multipart/byteranges` response, read one at a time with
/// [`ByteRangeResponse::next_part`].
pub struct ByteRangeResponse<P: DownloadReporter> {
    reader: BufReader<BodyReader>,
    reporter: P,
    boundary: Vec<u8>,
    errored: bool,
}

impl<P: DownloadReporter> ByteRangeResponse<P> {
    pub fn from_response(response: Response, reporter: P) -> anyhow::Result<Self> {
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .context("Content-Type missing")?;
        let boundary = content_type
            .to_str()?
            .split("boundary=")
            .nth(1)
            .context("Boundary missing")?
            .to_string();

        let body = StreamReader::new(response.bytes_stream().map_err(std::io::Error::other));

        Ok(Self::new(body, boundary.into_bytes(), reporter))
    }

    pub fn new(reader: impl AsyncRead + Send + 'static, boundary: Vec<u8>, reporter: P) -> Self {
        Self {
            reader: BufReader::new(Box::pin(reader)),
            reporter,
            boundary,
            errored: false,
        }
    }

    /// The next part's body, or `None` once the final boundary has been read.
    /// After an error every further call returns `None`.
    pub async fn next_part(&mut self) -> Option<anyhow::Result<Vec<u8>>> {
        if self.errored {
            return None;
        }

        match get_next_part(&mut self.reader, &self.boundary).await {
            Ok(Some(part)) => {
                self.reporter.report_progress(part.len() as u64);
                Some(Ok(part))
            }
            Ok(None) => None,
            Err(e) => {
                self.errored = true;
                Some(Err(e))
            }
        }
    }
}

async fn get_next_part<R: AsyncRead + Unpin>(
    reader: &mut BufReader<R>,
    boundary: &[u8],
) -> anyhow::Result<Option<Vec<u8>>> {
    let boundary_len = boundary.len() + 4; // "--" + boundary + "\r\n"
    let mut boundary_bytes = vec![0u8; boundary_len];
    let read = reader.read_exact(&mut boundary_bytes).await;

    if let Err(e) = read {
        return if e.kind() == std::io::ErrorKind::UnexpectedEof {
            Ok(None) // Expected EOF
        } else {
            Err(anyhow!("Failed to read boundary: {}", e))
        };
    }

    if &boundary_bytes[..2] != b"--" || &boundary_bytes[2..boundary_len - 2] != boundary {
        return Err(anyhow!("Invalid boundary"));
    }

    if &boundary_bytes[boundary_len - 2..] == b"--" {
        let mut check_eof = [0u8; 2];
        reader.read_exact(&mut check_eof).await?;

        if &check_eof[..2] != b"\r\n" {
            return Err(anyhow!("Invalid data after final boundary"));
        }

        let mut peek_buf = [0u8; 1];
        let bytes_peeked = reader.read(&mut peek_buf).await?;
        if bytes_peeked != 0 {
            return Err(anyhow!("Data found after final boundary"));
        }

        return Ok(None); // End of multipart
    }

    if &boundary_bytes[boundary_len - 2..] != b"\r\n" {
        return Err(anyhow!("Invalid boundary"));
    }

    let mut part_size = None;
    let mut counter = 0;

    loop {
        if counter > 50 {
            return Err(anyhow!("Too many headers without terminating CRLF"));
        }

        let mut header = Vec::new();
        let bytes_read = reader.read_until(b'\n', &mut header).await?;

        if bytes_read == 0 {
            return Err(anyhow!("Unexpected EOF while reading headers"));
        }

        if header == b"\r\n" {
            break;
        }

        let header_str = String::from_utf8_lossy(&header);
        if let Some(caps) = CONTENT_RANGE_HEADER.captures(&header_str.to_lowercase()) {
            if part_size.is_some() {
                return Err(anyhow!("Multiple content-range headers found"));
            }
            let start: usize = caps[1]
                .parse()
                .map_err(|_| anyhow!("Invalid content-range size"))?;
            let end: usize = caps[2]
                .parse()
                .map_err(|_| anyhow!("Invalid content-range size"))?;
            let size = end - start + 1;
            part_size = Some(size);
        }

        counter += 1;

        // Ignore other headers for now
    }

    // We are at the body

    let part_size = match part_size {
        Some(size) => size,
        None => return Err(anyhow!("Missing content-range header")),
    };

    let mut part_data = vec![0u8; part_size];
    reader
        .read_exact(&mut part_data)
        .await
        .context(anyhow!("Failed to read content of length: {}", part_size))?;

    // Read the trailing \r\n after the part
    let mut trailing = [0u8; 2];
    reader.read_exact(&mut trailing).await?;

    if &trailing != b"\r\n" {
        return Err(anyhow!("Invalid trailing after part"));
    }

    Ok(Some(part_data))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::progress_reporting::progress_reporter::NoopProgressReporter;

    fn response(body: &'static [u8]) -> ByteRangeResponse<NoopProgressReporter> {
        ByteRangeResponse::new(body, b"XYZ".to_vec(), NoopProgressReporter)
    }

    #[tokio::test]
    async fn reads_every_part_then_stops() {
        let mut parts = response(
            b"--XYZ\r\nContent-Range: bytes 0-2/10\r\n\r\nabc\r\n\
              --XYZ\r\nContent-Type: x\r\nContent-Range: bytes 5-6/10\r\n\r\nde\r\n\
              --XYZ--\r\n",
        );

        assert_eq!(parts.next_part().await.unwrap().unwrap(), b"abc");
        assert_eq!(parts.next_part().await.unwrap().unwrap(), b"de");
        assert!(parts.next_part().await.is_none());
    }

    #[tokio::test]
    async fn stops_after_the_first_error() {
        let mut parts = response(b"--NOPE\r\n");

        assert!(parts.next_part().await.unwrap().is_err());
        assert!(parts.next_part().await.is_none());
    }
}
