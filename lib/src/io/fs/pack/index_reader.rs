use crate::index::index_node::{FileKind, IndexNode, NodeKind};
use crate::io::fs::cache::file_cache_entry::FileCacheEntry;
use crate::io::fs::cache::kv_cache::KVCache;
use crate::io::name_consts::{get_pack_addon_directory_name, CACHE_DB_DIR_NAME};
use crate::io::rel_path::RelPath;
use crate::pack::pack_config::PackConfig;
use crate::pack::pack_index::PackIndex;
use anyhow::{anyhow, Result};
use bi_fs_rs::pbo::handle::PBOHandle;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use regex::Regex;
use std::fmt::Debug;
use std::fs::{DirEntry, ReadDir};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

static PBO_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(.+)\.pbo$").unwrap());
static ADDON_FOLDER_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^@.*$").unwrap());

pub struct IndexReader {
    addon_dir: PathBuf,
    cache: KVCache,
}

impl IndexReader {
    pub fn from_config(config: &PackConfig, base_dir: PathBuf) -> Result<Self> {
        let addon_dir = base_dir.join(get_pack_addon_directory_name(&config.name));

        if !addon_dir.is_dir() {
            anyhow::bail!("Path is not a directory: {:?}", addon_dir);
        }

        let cache_dir = addon_dir.join(CACHE_DB_DIR_NAME);
        let cache = KVCache::new(cache_dir)?;

        Ok(Self { addon_dir, cache })
    }

    pub fn index_addons(&self) -> Result<PackIndex> {
        let addon_folders = std::fs::read_dir(&self.addon_dir)?;

        let base_path = RelPath::new();

        let rel_paths = addon_folders
            .filter_map(Result::ok)
            .map(|f| file_name_to_string(f.path()))
            .filter(|f| ADDON_FOLDER_REGEX.is_match(f))
            .map(|folder_name| base_path.push(&folder_name))
            .collect::<Vec<_>>();

        let indexes = rel_paths
            .into_par_iter()
            .map(|p| self.index_dir(p))
            .collect::<Result<Vec<_>>>()?;

        Ok(PackIndex(indexes))
    }

    fn index_dir(&self, rel_path: RelPath) -> Result<IndexNode> {
        let fs_path = rel_path.with_base_path(&self.addon_dir);

        if !fs_path.is_dir() {
            anyhow::bail!("Path is not a directory: {:?}", fs_path);
        }

        let name = file_name_to_string(&fs_path);

        let entries = std::fs::read_dir(&fs_path)?;
        let sorted_entries = sort_folders(entries)?;

        let folder_indexes = sorted_entries
            .into_par_iter()
            .map(|entry| {
                let entry_name = file_name_to_string(&entry.path());
                let rel_entry_path = rel_path.push(&entry_name);
                if entry.path().is_file() {
                    self.index_file(rel_entry_path)
                } else if entry.path().is_dir() {
                    self.index_dir(rel_entry_path)
                } else {
                    unreachable!("Path is neither file nor directory: {:?}", entry_name);
                }
            })
            .collect::<Result<Vec<IndexNode>>>()?;

        let mut hasher = blake3::Hasher::new();
        hasher.update(name.as_bytes());
        for part in &folder_indexes {
            hasher.update(&part.checksum);
        }
        let checksum = hasher.finalize().as_bytes().to_vec();

        Ok(IndexNode {
            name,
            checksum,
            kind: NodeKind::Folder(folder_indexes),
        })
    }

    fn index_file(&self, rel_path: RelPath) -> Result<IndexNode> {
        let fs_path = rel_path.with_base_path(&self.addon_dir);

        if !fs_path.is_file() {
            anyhow::bail!("Path is not a file: {:?}", fs_path);
        }

        let metadata = std::fs::metadata(&fs_path)?;
        let last_modified = metadata
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        let length = metadata.len();

        read_file_to_part(fs_path, &self.cache)
    }
}

fn file_name_to_string<P: AsRef<Path> + Debug>(fs_path: P) -> String {
    fs_path
        .as_ref()
        .file_name()
        .and_then(|s| s.to_str())
        .expect("Bad path format")
        .to_owned()
}

fn sort_folders(read_dir: ReadDir) -> Result<Vec<DirEntry>> {
    let mut entries: Vec<_> = read_dir.collect::<Result<_, _>>()?;
    entries.sort_by_key(|e| file_name_to_string(&e.file_name()));
    Ok(entries)
}

fn read_file_to_part(fs_path: PathBuf, cache: &KVCache) -> Result<IndexNode> {
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
        return Ok(old_part.index);
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
        index: part.clone(),
    };

    cache.set(path_to_key(&fs_path)?, file_cache)?;

    Ok(part)
}

fn read_pbo_to_part(fs_path: &PathBuf) -> Result<IndexNode> {
    if !fs_path.is_file() {
        anyhow::bail!("Path is not a file: {:?}", fs_path);
    }

    let rel_path = fs_path.file_name().unwrap().to_str().unwrap().to_owned();

    let mut pbo_handle = PBOHandle::open_file(fs_path)?;

    IndexNode::from_handle(&mut pbo_handle, &rel_path)
}

fn read_generic_file_to_part(fs_path: &PathBuf) -> Result<IndexNode> {
    if !fs_path.is_file() {
        anyhow::bail!("Path is not a file: {:?}", fs_path);
    }

    let file_name = fs_path.file_name().unwrap().to_str().unwrap().to_owned();

    let data = std::fs::read(fs_path)?;
    let length = data.len() as u64;
    let mut hasher = blake3::Hasher::new();
    hasher.update(&data);
    hasher.update(file_name.as_bytes());
    let checksum = hasher.finalize().as_bytes().to_vec();

    let last_modified = std::fs::metadata(fs_path)
        .and_then(|meta| meta.modified())
        .map(|time| {
            time.duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        })
        .unwrap_or(0);

    Ok(IndexNode {
        name: file_name,
        checksum,
        kind: NodeKind::File {
            last_modified,
            length,
            kind: FileKind::Generic,
        },
    })
}

pub fn path_to_key(path: &Path) -> anyhow::Result<String> {
    Ok(path
        .to_str()
        .ok_or(anyhow!("invalid path: {:?}", path))?
        .to_owned())
}
