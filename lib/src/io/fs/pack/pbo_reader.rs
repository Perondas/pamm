use crate::index::index_node::{FileKind, IndexNode, NodeKind, PBOPart};
use anyhow::Result;
use bi_fs_rs::pbo::handle::PBOHandle;
use std::io::{Read, Seek, SeekFrom};

impl IndexNode {
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
            let mut file_hasher = blake3::Hasher::new();
            file_hasher.update(&data);
            let file_checksum = file_hasher.finalize().as_bytes().to_vec();

            parts.push(PBOPart {
                name: file.filename.to_string(),
                length: file.size,
                checksum: file_checksum,
                start_offset: offset,
            });
            offset += file.size as u64;
        }

        // Compute PBO checksum
        let mut pbo_hasher = blake3::Hasher::new();
        pbo_hasher.update(&handle.checksum.data);
        for part in &parts {
            pbo_hasher.update(&part.checksum);
        }
        pbo_hasher.update(rel_path.as_bytes());
        let pbo_checksum = pbo_hasher.finalize().as_bytes().to_vec();

        let last_modified = handle
            .handle
            .metadata()
            .and_then(|meta| meta.modified())
            .map(|time| {
                time.duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            })
            .unwrap_or(0);

        Ok(Self {
            name: rel_path.to_string(),
            checksum: pbo_checksum,
            kind: NodeKind::File {
                last_modified,
                length: handle.length,
                kind: FileKind::Pbo {
                    parts,
                    blob_offset: handle.blob_start,
                },
            },
        })
    }
}
