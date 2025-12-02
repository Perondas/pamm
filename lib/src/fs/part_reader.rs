use crate::fs::cache::file_cache_entry::FileCacheEntry;
use crate::fs::cache::kv_cache::{KVCache, path_to_key};
use crate::pack::manifest::entries::manifest_entry::FileKind::Generic;
use crate::pack::manifest::entries::manifest_entry::{EntryKind, ManifestEntry};
use anyhow::{Result, anyhow};
use bi_fs_rs::pbo::handle::PBOHandle;
use regex::Regex;
use sha1::{Digest, Sha1};
use std::path::PathBuf;
use std::sync::LazyLock;

static PBO_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(.+)\.pbo$").unwrap());

pub fn read_to_part(path_buf: PathBuf, cache: &KVCache) -> Result<ManifestEntry> {
    if path_buf.is_file() {
        read_file_to_part(path_buf, cache)
    } else if path_buf.is_dir() {
        read_dir_to_parts(path_buf, cache)
    } else {
        panic!("Path is neither file nor directory");
    }
}

fn read_dir_to_parts(fs_path: PathBuf, cache: &KVCache) -> Result<ManifestEntry> {
    if !fs_path.is_dir() {
        anyhow::bail!("Path is not a directory: {:?}", fs_path);
    }

    let name = fs_path.file_name().unwrap().to_str().unwrap().to_owned();

    let mut folder_parts = Vec::new();

    let entries = std::fs::read_dir(&fs_path)?;
    let sorted_entries = {
        let mut entries: Vec<_> = entries.collect::<Result<_, _>>()?;
        entries.sort_by_key(|e| e.path());
        entries
    };

    for entry in sorted_entries {
        let entry_path = entry.path();
        if entry_path.is_file() {
            folder_parts.push(read_file_to_part(entry_path, cache)?);
        } else if entry_path.is_dir() {
            folder_parts.push(read_dir_to_parts(entry_path, cache)?);
        }
    }

    let mut hasher = Sha1::new();
    sha1::Digest::update(&mut hasher, name.as_bytes());
    for part in &folder_parts {
        sha1::Digest::update(&mut hasher, &part.checksum);
    }
    let checksum = hasher.finalize().to_vec();

    Ok(ManifestEntry {
        name,
        checksum,
        kind: EntryKind::Folder(folder_parts),
    })
}

fn read_file_to_part(fs_path: PathBuf, cache: &KVCache) -> Result<ManifestEntry> {
    let metadata = std::fs::metadata(&fs_path)?;
    let last_modified = metadata
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let length = metadata.len();

    let old_part: Option<FileCacheEntry> = cache.get(path_to_key(&fs_path)?)?;

    if let Some(old_part) = old_part
        && old_part.last_modified == last_modified
        && old_part.length == length
    {
        return Ok(old_part.part);
    }

    let part = if PBO_NAME_REGEX.is_match(
        fs_path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or(anyhow!("Bad path: {:?}", fs_path))?,
    ) {
        match read_pbo_to_part(&fs_path) {
            Ok(part) => Ok(part),
            Err(e) => {
                println!(
                    "Warning: Failed to read PBO file {:?}, falling back to generic file. Error: {}",
                    fs_path, e
                );
                read_generic_file_to_part(&fs_path)
            }
        }
    } else {
        read_generic_file_to_part(&fs_path)
    }?;

    let file_cache = FileCacheEntry {
        last_modified,
        length,
        part: part.clone(),
    };

    cache.set(path_to_key(&fs_path)?, file_cache)?;

    Ok(part)
}

fn read_pbo_to_part(fs_path: &PathBuf) -> Result<ManifestEntry> {
    if !fs_path.is_file() {
        anyhow::bail!("Path is not a file: {:?}", fs_path);
    }

    let rel_path = fs_path.file_name().unwrap().to_str().unwrap().to_owned();

    let mut pbo_handle = PBOHandle::open_file(fs_path)?;

    ManifestEntry::from_handle(&mut pbo_handle, &rel_path)
}

fn read_generic_file_to_part(fs_path: &PathBuf) -> Result<ManifestEntry> {
    if !fs_path.is_file() {
        anyhow::bail!("Path is not a file: {:?}", fs_path);
    }

    let file_name = fs_path.file_name().unwrap().to_str().unwrap().to_owned();

    let data = std::fs::read(fs_path)?;
    let length = data.len() as u64;
    let mut hasher = Sha1::new();
    sha1::Digest::update(&mut hasher, data);
    sha1::Digest::update(&mut hasher, file_name.as_bytes());
    let checksum = hasher.finalize().to_vec();

    let last_modified = std::fs::metadata(fs_path)
        .and_then(|meta| meta.modified())
        .map(|time| {
            time.duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        })
        .unwrap_or(0);

    Ok(ManifestEntry {
        name: file_name,
        checksum,
        kind: EntryKind::File {
            last_modified,
            length,
            kind: Generic,
        },
    })
}
