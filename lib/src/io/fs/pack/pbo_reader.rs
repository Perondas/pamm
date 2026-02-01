use crate::index::index_node::{FileKind, IndexNode, NodeKind, PBOPart};
use anyhow::{ensure, Result};
use bi_fs_rs::pbo::handle::PBOHandle;
use std::io::{Read, Seek, SeekFrom};

impl IndexNode {
    pub fn from_handle(handle: &mut PBOHandle, file_name: &str) -> Result<Self> {
        let mut offset = 0;
        let mut parts = Vec::with_capacity(handle.files.len());

        // Seek to the start of the blob
        handle.handle.seek(SeekFrom::Start(handle.blob_start))?;

        for file in &handle.files {
            // Read Data
            let mut data = vec![0; file.size as usize];
            handle.handle.read_exact(&mut data)?;

            // Hash Data
            let mut file_hasher = blake3::Hasher::new();
            file_hasher.update(&data);
            file_hasher.update(&file.filename.0);
            file_hasher.update(&file.size.to_le_bytes());
            let file_checksum = file_hasher.finalize().as_bytes().to_vec();

            parts.push(PBOPart {
                name: file.filename.to_string(),
                length: file.size,
                checksum: file_checksum,
                start_offset: offset,
            });
            offset += file.size as u64;
        }

        // Skip past the checksum
        handle.handle.seek_relative(20)?;

        // We should be at the end of the file
        ensure!(
            handle.handle.stream_position()? == handle.length,
            "File is not at the end"
        );

        // Compute PBO checksum
        let mut pbo_hasher = blake3::Hasher::new();
        pbo_hasher.update(&handle.checksum.data);
        for part in &parts {
            pbo_hasher.update(&part.checksum);
        }
        pbo_hasher.update(file_name.as_bytes());
        let pbo_checksum = pbo_hasher.finalize().as_bytes().to_vec();

        // Header len + 1 + blob + checksum
        let computed_len: u64 =
            (handle.blob_start) + parts.iter().map(|p| p.length as u64).sum::<u64>() + 20;

        ensure!(
            computed_len == handle.length,
            "Computed PBO length does not match handle length. Computed: {}, Handle: {}",
            computed_len,
            handle.length
        );

        Ok(Self {
            name: file_name.to_string(),
            checksum: pbo_checksum,
            kind: NodeKind::File {
                length: handle.length,
                kind: FileKind::Pbo {
                    parts,
                    blob_start: handle.blob_start,
                },
            },
        })
    }
}
