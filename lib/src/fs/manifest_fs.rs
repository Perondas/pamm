use crate::fs::cache::kv_cache::KVCache;
use crate::fs::part_reader::read_to_part;
use crate::manifest::entries::manifest_entry::ManifestEntry;
use crate::manifest::pack_manifest::PackManifest;
use crate::name_consts::{CACHE_DB_DIR_NAME, get_pack_addon_directory_name};
use rayon::prelude::*;
use sha1::{Digest, Sha1};
use std::path::{Path, PathBuf};

impl PackManifest {
    pub fn gen_from_fs(base_path: &Path, name: &str, force_refresh: bool) -> anyhow::Result<Self> {
        let cache = KVCache::new(base_path.join(CACHE_DB_DIR_NAME))?;
        if force_refresh {
            cache.remove_all()?;
        }

        let addons =
            read_addons_to_part(base_path.join(get_pack_addon_directory_name(name)), &cache)?;

        let mut hasher = Sha1::new();
        for addon in addons.iter() {
            sha1::Digest::update(&mut hasher, &addon.checksum);
        }
        let pack_checksum = hasher.finalize().to_vec();

        Ok(Self {
            name: name.to_string(),
            repo_checksum: pack_checksum,
            addons,
        })
    }

    /*    pub fn get_addon_paths(&self, base_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
        let required_dir = base_path.join(ADDON_DIR_NAME);
        let res = self
            .addons
            .iter()
            .map(|part| required_dir.join(&part.name).canonicalize())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(res)
    }*/
}

fn read_addons_to_part(folder: PathBuf, cache: &KVCache) -> anyhow::Result<Vec<ManifestEntry>> {
    if !folder.is_dir() {
        anyhow::bail!("{} is not a directory", folder.display());
    }

    let entries: Vec<_> = std::fs::read_dir(folder)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();

    let mut parts: Vec<ManifestEntry> = entries
        .par_iter()
        .filter_map(|path| {
            if path.is_dir() {
                Some(read_to_part(path.clone(), cache))
            } else {
                eprintln!("{} is not a directory", path.display());
                None
            }
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    parts.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(parts)
}
