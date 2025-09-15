use crate::pack::pack_part::generic_file::GenericFile;
use crate::pack::pack_part::part::File::{Generic, PBO};
use crate::pack::pack_part::part::PackPart;
use crate::pack::pack_part::pbo_file::PBOFile;
use anyhow::Result;
use bi_fs_rs::pbo::handle::PBOHandle;
use regex::Regex;
use sha1::{Digest, Sha1};
use std::path::PathBuf;
use std::sync::LazyLock;
use crate::pack::pack_part::folder::Folder;

static PBO_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(.+)\.pbo$").unwrap());

pub fn read_to_part(path_buf: PathBuf, url_path: &str) -> Result<PackPart> {
    if path_buf.is_file() {
        read_file_to_part(path_buf, url_path)
    } else if path_buf.is_dir() {
        read_dir_to_parts(path_buf, url_path)
    } else {
        panic!("Path is neither file nor directory");
    }
}

fn read_dir_to_parts(fs_path: PathBuf, url_path: &str) -> Result<PackPart> {
    if !fs_path.is_dir() {
        anyhow::bail!("Path is not a directory: {:?}", fs_path);
    }

    let rel_path = add_file_name_to_url_path(&fs_path, url_path)?;

    let mut folder_parts = Vec::new();

    for entry in std::fs::read_dir(&fs_path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            folder_parts.push(read_file_to_part(entry_path, &rel_path)?);
        } else if entry_path.is_dir() {
            folder_parts.push(read_dir_to_parts(entry_path, &rel_path)?);
        }
    }

    let mut hasher = Sha1::new();
    sha1::Digest::update(&mut hasher, rel_path.as_bytes());
    for part in &folder_parts {
        sha1::Digest::update(&mut hasher, part.get_checksum());
    }
    let checksum = hasher.finalize().to_vec();


    Ok(PackPart::Folder(Folder {
        rel_path,
        checksum,
        children: folder_parts,
    }))
}

fn read_file_to_part(path_buf: PathBuf, url_path: &str) -> Result<PackPart> {
    if PBO_NAME_REGEX.is_match(path_buf.file_name().unwrap().to_str().unwrap()) {
        read_pbo_to_part(path_buf, url_path)
    } else {
        read_generic_file_to_part(path_buf, url_path)
    }
}

fn read_pbo_to_part(fs_path: PathBuf, url_path: &str) -> Result<PackPart> {
    if !fs_path.is_file() {
        anyhow::bail!("Path is not a file: {:?}", fs_path);
    }

    let rel_path = add_file_name_to_url_path(&fs_path, url_path)?;

    let mut pbo_handle = PBOHandle::open_file(&fs_path)?;
    let file = PBOFile::from_handle(&mut pbo_handle, &rel_path)?;
    Ok(PackPart::File(PBO(file)))
}

fn read_generic_file_to_part(fs_path: PathBuf, url_path: &str) -> Result<PackPart> {
    if !fs_path.is_file() {
        anyhow::bail!("Path is not a file: {:?}", fs_path);
    }

    let rel_path = add_file_name_to_url_path(&fs_path, url_path)?;

    let data = std::fs::read(&fs_path)?;
    let length = data.len() as u64;
    let mut hasher = Sha1::new();
    sha1::Digest::update(&mut hasher, data);
    sha1::Digest::update(&mut hasher, rel_path.as_bytes());
    let checksum = hasher.finalize().to_vec();

    let last_modified = std::fs::metadata(fs_path)
        .and_then(|meta| meta.modified())
        .map(|time| time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
        .unwrap_or(0);

    Ok(PackPart::File(Generic(GenericFile {
        rel_path,
        last_modified,
        length,
        checksum,
    })))
}

fn add_file_name_to_url_path(fs_path: &PathBuf, url_path: &str) -> Result<String> {
    let file_name = fs_path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Failed to get file name from path: {:?}", fs_path))?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to convert file name to string: {:?}", fs_path))?;
    let rel_path = if url_path.ends_with('/') {
        format!("{}{}", url_path, file_name)
    } else {
        format!("{}/{}", url_path, file_name)
    };
    Ok(rel_path)
}
