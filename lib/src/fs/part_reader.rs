use crate::pack::pack_part::folder::Folder;
use crate::pack::pack_part::generic_file::GenericFile;
use crate::pack::pack_part::part::File::{Generic, PBO};
use crate::pack::pack_part::part::{File, PackPart};
use crate::pack::pack_part::pbo_file::PBOFile;
use anyhow::Result;
use bi_fs_rs::pbo::handle::PBOHandle;
use regex::Regex;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;

static PBO_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(.+)\.pbo$").unwrap());

pub fn read_to_part(path_buf: PathBuf, old_index: Option<&PackPart>) -> Result<PackPart> {
    if path_buf.is_file() {
        let old_index = old_index.and_then(|part| match part {
            PackPart::File(file) => Some(file),
            _ => None,
        });
        read_file_to_part(path_buf, old_index)
    } else if path_buf.is_dir() {
        let old_index = old_index.and_then(|part| match part {
            PackPart::Folder(folder) => Some(folder),
            _ => None,
        });
        read_dir_to_parts(path_buf, old_index)
    } else {
        panic!("Path is neither file nor directory");
    }
}

fn read_dir_to_parts(fs_path: PathBuf, old_index: Option<&Folder>) -> Result<PackPart> {
    if !fs_path.is_dir() {
        anyhow::bail!("Path is not a directory: {:?}", fs_path);
    }

    let name = fs_path.file_name().unwrap().to_str().unwrap().to_owned();

    let mut folder_parts = Vec::new();

    let mut old_child_folders = if let Some(old_folder) = &old_index {
        old_folder
            .children
            .iter()
            .filter_map(|part| match part {
                PackPart::Folder(folder) => Some((folder.name.clone(), folder.to_owned())),
                _ => None,
            })
            .collect::<HashMap<_, _>>()
    } else {
        HashMap::new()
    };

    let mut old_child_files = if let Some(old_folder) = &old_index {
        old_folder
            .children
            .iter()
            .filter_map(|part| match part {
                PackPart::File(file) => Some((file.get_rel_path().to_owned(), file.to_owned())),
                _ => None,
            })
            .collect::<HashMap<_, _>>()
    } else {
        HashMap::new()
    };

    for entry in std::fs::read_dir(&fs_path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let child_name = entry.file_name().to_str().unwrap().to_owned();
        if entry_path.is_file() {
            folder_parts.push(read_file_to_part(
                entry_path,
                old_child_files.remove(&child_name).as_ref(),
            )?);
        } else if entry_path.is_dir() {
            folder_parts.push(read_dir_to_parts(
                entry_path,
                old_child_folders.remove(&child_name).as_ref(),
            )?);
        }
    }

    let mut hasher = Sha1::new();
    sha1::Digest::update(&mut hasher, name.as_bytes());
    for part in &folder_parts {
        sha1::Digest::update(&mut hasher, part.get_checksum());
    }
    let checksum = hasher.finalize().to_vec();

    Ok(PackPart::Folder(Folder {
        name: name,
        checksum,
        children: folder_parts,
    }))
}

fn read_file_to_part(path_buf: PathBuf, old_index: Option<&File>) -> Result<PackPart> {
    if PBO_NAME_REGEX.is_match(path_buf.file_name().unwrap().to_str().unwrap()) {
        let old_index = old_index.and_then(|file| match file {
            PBO(pbo) => Some(pbo),
            _ => None,
        });
        read_pbo_to_part(path_buf, old_index)
    } else {
        let old_index = old_index.and_then(|file| match file {
            Generic(generic) => Some(generic),
            _ => None,
        });
        read_generic_file_to_part(path_buf, old_index)
    }
}

fn read_pbo_to_part(fs_path: PathBuf, old_part: Option<&PBOFile>) -> Result<PackPart> {
    if !fs_path.is_file() {
        anyhow::bail!("Path is not a file: {:?}", fs_path);
    }

    let rel_path = fs_path.file_name().unwrap().to_str().unwrap().to_owned();

    if let Some(old_part) = old_part {
        let metadata = std::fs::metadata(&fs_path)?;
        let last_modified = metadata
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        let length = metadata.len();
        if old_part.last_modified == last_modified && old_part.length == length {
            return Ok(PackPart::File(PBO(old_part.clone())));
        }
    }

    let mut pbo_handle = PBOHandle::open_file(&fs_path)?;
    let file = PBOFile::from_handle(&mut pbo_handle, &rel_path)?;
    Ok(PackPart::File(PBO(file)))
}

fn read_generic_file_to_part(fs_path: PathBuf, old_part: Option<&GenericFile>) -> Result<PackPart> {
    if !fs_path.is_file() {
        anyhow::bail!("Path is not a file: {:?}", fs_path);
    }

    let file_name = fs_path.file_name().unwrap().to_str().unwrap().to_owned();

    if let Some(old_part) = old_part {
        let metadata = std::fs::metadata(&fs_path)?;
        let last_modified = metadata
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        let length = metadata.len();
        if old_part.last_modified == last_modified && old_part.length == length {
            return Ok(PackPart::File(Generic(old_part.clone())));
        }
    }

    let data = std::fs::read(&fs_path)?;
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

    Ok(PackPart::File(Generic(GenericFile {
        name: file_name,
        last_modified,
        length,
        checksum,
    })))
}
