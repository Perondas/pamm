use anyhow::Context;
use anyhow::anyhow;
use regex::Regex;
use std::io::{BufRead, BufReader, Read};
use std::sync::LazyLock;
use ureq::http::Response;
use ureq::http::header::CONTENT_TYPE;
use ureq::{Body, BodyReader};

static CONTENT_RANGE_HEADER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^content-range: bytes (\d+)-(\d+)/(\d+)\r\n$").unwrap());

pub struct ByteRangeResponse<R: Read> {
    reader: BufReader<R>,
    boundary: Vec<u8>,
    errored: bool,
}

pub trait IntoByteRangeResponse<R: Read> {
    fn into_byte_range_response(self) -> anyhow::Result<ByteRangeResponse<R>>;
}

impl IntoByteRangeResponse<BodyReader<'static>> for Response<Body> {
    fn into_byte_range_response(self) -> anyhow::Result<ByteRangeResponse<BodyReader<'static>>> {
        let (p, b) = self.into_parts();
        let content_type = p.headers.get(CONTENT_TYPE).expect("Content-Type missing");
        let boundary = content_type
            .to_str()?
            .split("boundary=")
            .nth(1)
            .expect("Boundary missing")
            .to_string();
        Ok(ByteRangeResponse::new(
            b.into_reader(),
            boundary.as_bytes().to_vec(),
        ))
    }
}

impl<R: Read> ByteRangeResponse<R> {
    pub fn new(reader: R, boundary: Vec<u8>) -> Self {
        Self {
            reader: BufReader::new(reader),
            boundary,
            errored: false,
        }
    }
}

impl<R: Read> Iterator for ByteRangeResponse<R> {
    type Item = anyhow::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.errored {
            return None;
        }

        match get_next_part(&mut self.reader, &self.boundary) {
            Ok(Some(part)) => Some(Ok(part)),
            Ok(None) => None,
            Err(e) => {
                self.errored = true;
                Some(Err(e))
            }
        }
    }
}

fn get_next_part<R: Read>(
    reader: &mut BufReader<R>,
    boundary: &[u8],
) -> anyhow::Result<Option<Vec<u8>>> {
    let boundary_len = boundary.len() + 4; // "--" + boundary + "\r\n"
    let mut boundary_bytes = vec![0u8; boundary_len];
    let read = reader.read_exact(&mut boundary_bytes);

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
        reader.read_exact(&mut check_eof)?;

        if &check_eof[..2] != b"\r\n" {
            return Err(anyhow!("Invalid data after final boundary"));
        }

        let mut peek_buf = [0u8; 1];
        let bytes_peeked = reader.read(&mut peek_buf)?;
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
        let bytes_read = reader.read_until(b'\n', &mut header)?;

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
        .context(anyhow!("Failed to read content of length: {}", part_size))?;

    // Read the trailing \r\n after the part
    let mut trailing = [0u8; 2];
    reader.read_exact(&mut trailing)?;

    if &trailing != b"\r\n" {
        return Err(anyhow!("Invalid trailing after part"));
    }

    Ok(Some(part_data))
}
