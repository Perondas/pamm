use crate::io::progress_reporting::download_reporter::DownloadReporter;
use log::debug;
use std::io::{Read, Write};

pub fn copy<R, W>(
    reader: &mut R,
    writer: &mut W,
    reporter: &impl DownloadReporter,
) -> std::io::Result<u64>
where
    R: Read + ?Sized,
    W: Write + ?Sized,
{
    // Read 1024 chunks and report them
    let mut total = 0;
    let mut buffer = [0_u8; 1024];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        total += bytes_read as u64;
        if bytes_read == 0 {
            debug!(
                "Finished copying with progress report, total bytes copied: {}",
                total
            );
            return Ok(total);
        }

        writer.write_all(&buffer[..bytes_read])?;
        reporter.report_progress(bytes_read as u64);
    }
}
