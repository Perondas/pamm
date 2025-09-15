use std::io::{Read, Seek, SeekFrom};
use serde::{Deserialize, Serialize};
use bi_fs_rs::pbo::handle::PBOHandle;
use anyhow::{Result};
use sha1::Digest;

#[derive(Debug, Serialize, Deserialize)]
pub struct PBOFile {
    pub rel_path: String,
    pub last_modified: u64,
    pub length: u64,
    pub checksum: Vec<u8>,
    pub blob_offset: u64,
    pub parts: Vec<PBOPart>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PBOPart {
    pub rel_path: String,
    pub length: u32,
    pub checksum: Vec<u8>,
    pub start_offset: u64,
}

impl PBOFile {
    pub fn from_handle(handle: &mut PBOHandle, rel_path: &str) -> Result<Self> {
        let mut offset = 0;
        let mut parts = Vec::with_capacity(handle.files.len());

        // Seek to the start of the blob
        handle.handle.seek(SeekFrom::Start(handle.blob_start))?;

        for file in &handle.files {
            // Read Data
            let mut data = vec![0; file.size as usize];
            handle.handle.read_exact(&mut data)?;

            // Hash Data
            let mut file_hasher = sha1::Sha1::new();
            sha1::Digest::update(&mut file_hasher, &data);
            let file_checksum = file_hasher.finalize().to_vec();

            parts.push(PBOPart {
                rel_path: file.filename.to_string(),
                length: file.size,
                checksum: file_checksum,
                start_offset: offset,
            });
            offset += file.size as u64;
        }

        // Compute PBO checksum
        let mut pbo_hasher = sha1::Sha1::new();
        sha1::Digest::update(&mut pbo_hasher, handle.checksum.data);
        for part in &parts {
            sha1::Digest::update(&mut pbo_hasher, &part.checksum);
        }
        sha1::Digest::update(&mut pbo_hasher, rel_path.as_bytes());
        let pbo_checksum = pbo_hasher.finalize().to_vec();

        let last_modified = std::fs::metadata(rel_path)
            .and_then(|meta| meta.modified())
            .map(|time| time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
            .unwrap_or(0);

        Ok(Self {
            rel_path: rel_path.to_string(),
            length: handle.length,
            checksum:pbo_checksum,
            last_modified,
            parts,
            blob_offset: handle.blob_start
        })
    }
}

