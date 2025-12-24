use crate::index::index_node::FileKind;
use crate::index::index_node::IndexNode;
use crate::index::index_node::NodeKind;
use crate::io::fs::cache::file_cache_entry::FileCacheEntry;
use crate::io::fs::cache::kv_cache::KVCache;
use crate::io::name_consts::CACHE_DB_DIR_NAME;
use crate::io::name_consts::get_pack_addon_directory_name;
use crate::io::rel_path::RelPath;
use crate::pack::pack_config::PackConfig;
use crate::pack::pack_index::PackIndex;
use anyhow::{Result, anyhow};
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
    pack_name: String,
}

impl IndexReader {
    pub fn from_config(config: &PackConfig, base_dir: PathBuf) -> Result<Self> {
        let addon_dir = base_dir.join(get_pack_addon_directory_name(&config.name));

        if !addon_dir.is_dir() {
            anyhow::bail!("Path is not a directory: {:?}", addon_dir);
        }

        let cache_dir = addon_dir.join(CACHE_DB_DIR_NAME);
        let cache = KVCache::new(cache_dir)?;

        Ok(Self {
            addon_dir,
            cache,
            pack_name: config.name.clone(),
        })
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

        Ok(PackIndex {
            addons: indexes,
            pack_name: self.pack_name.clone(),
        })
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
                let entry_name = file_name_to_string(entry.file_name());
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

        let stored_node = self.cache.get::<_, FileCacheEntry>(&rel_path.as_str())?;

        if let Some(FileCacheEntry {
            length: cached_length,
            last_modified: cached_last_modified,
            index:
                node @ IndexNode {
                    kind: NodeKind::File { .. },
                    ..
                },
        }) = stored_node
            && cached_last_modified == last_modified
            && cached_length == length
        {
            return Ok(node);
        }

        let index = if PBO_NAME_REGEX.is_match(
            fs_path
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or(anyhow!("Bad path: {:?}", fs_path))?,
        ) {
            match index_pbo(&fs_path) {
                Ok(part) => Ok(part),
                Err(e) => {
                    println!(
                        "Warning: Failed to read PBO file {:?}, falling back to generic file. Error: {}",
                        fs_path, e
                    );
                    index_generic_file(&fs_path)
                }
            }
        } else {
            index_generic_file(&fs_path)
        }?;

        let file_cache = FileCacheEntry {
            last_modified,
            length,
            index: index.clone(),
        };

        self.cache.set(rel_path.as_str(), file_cache)?;

        Ok(index)
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
    entries.sort_by_key(|e| file_name_to_string(e.file_name()));
    Ok(entries)
}

fn index_pbo(fs_path: &PathBuf) -> Result<IndexNode> {
    if !fs_path.is_file() {
        anyhow::bail!("Path is not a file: {:?}", fs_path);
    }

    let name = file_name_to_string(fs_path);

    let mut pbo_handle = PBOHandle::open_file(fs_path)?;

    IndexNode::from_handle(&mut pbo_handle, &name)
}

fn index_generic_file(fs_path: &PathBuf) -> Result<IndexNode> {
    if !fs_path.is_file() {
        anyhow::bail!("Path is not a file: {:?}", fs_path);
    }

    let file_name = file_name_to_string(fs_path);

    let mut file = std::fs::File::open(fs_path)?;
    let mut hasher = blake3::Hasher::new();
    let length = std::io::copy(&mut file, &mut hasher)?;
    hasher.update(file_name.as_bytes());
    let checksum = hasher.finalize().as_bytes().to_vec();

    Ok(IndexNode {
        name: file_name,
        checksum,
        kind: NodeKind::File {
            length,
            kind: FileKind::Generic,
        },
    })
}
