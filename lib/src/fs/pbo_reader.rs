use crate::pack::manifest::entries::manifest_entry::{EntryKind, FileKind, ManifestEntry, PBOPart};
use anyhow::Result;
use bi_fs_rs::pbo::handle::PBOHandle;
use sha1::Digest;
use std::io::{Read, Seek, SeekFrom};

impl ManifestEntry {
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
                name: file.filename.to_string(),
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
            kind: EntryKind::File {
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
