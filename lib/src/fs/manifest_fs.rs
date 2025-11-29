use crate::bin_serialization::{from_reader, to_writer};
use crate::consts::{CACHE_DB_DIR_NAME, MANIFEST_FILE_NAME, OPTIONAL_DIR_NAME, REQUIRED_DIR_NAME};
use crate::fs::part_reader::read_to_part;
use crate::kv_cache::KVCache;
use crate::pack::pack_manifest::PackManifest;
use crate::pack::pack_part::part::PackPart;
use sha1::{Digest, Sha1};
use std::fs;
use std::path::{Path, PathBuf};

impl PackManifest {
    pub fn gen_from_fs(base_path: &Path, force_refresh: bool) -> anyhow::Result<Self> {
        let cache = KVCache::new(base_path.join(CACHE_DB_DIR_NAME))?;
        if force_refresh {
            cache.remove_all()?;
        }

        let required_addons = read_addons_to_part(base_path.join(REQUIRED_DIR_NAME), &cache)?;
        let optional_addons = read_addons_to_part(base_path.join(OPTIONAL_DIR_NAME), &cache)?;

        let all_addons = required_addons.iter().chain(optional_addons.iter());

        let mut hasher = Sha1::new();
        for addon in all_addons {
            sha1::Digest::update(&mut hasher, addon.get_checksum());
        }
        let pack_checksum = hasher.finalize().to_vec();

        Ok(Self {
            pack_checksum,
            required_addons,
            optional_addons,
        })
    }

    pub fn read(base_path: &Path) -> anyhow::Result<Self> {
        let path = base_path.join(MANIFEST_FILE_NAME);
        if path.exists() {
            let mut file = fs::File::open(&path)?;
            from_reader(&mut file)
        } else {
            anyhow::bail!("no manifest file found")
        }
    }

    pub fn read_or_default(base_path: &Path) -> anyhow::Result<Self> {
        let path = base_path.join(MANIFEST_FILE_NAME);
        let stored_manifest = if path.exists() {
            let mut file = fs::File::open(&path)?;
            from_reader(&mut file)?
        } else {
            println!("No pack found in the current directory.");
            println!("Reinitializing a new pack manifest.");
            PackManifest::default()
        };

        Ok(stored_manifest)
    }

    pub fn write_to_fs(&self, base_path: &Path) -> anyhow::Result<()> {
        let path = base_path.join(MANIFEST_FILE_NAME);
        let mut file = fs::File::create(&path)?;
        to_writer(&mut file, self)?;
        Ok(())
    }

    pub fn get_required_addon_paths(&self, base_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
        let required_dir = base_path.join(REQUIRED_DIR_NAME);
        let res = self
            .required_addons
            .iter()
            .map(|part| required_dir.join(part.get_name()).canonicalize())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(res)
    }

    pub fn get_optional_addon_paths(&self, base_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
        let optional_dir = base_path.join(OPTIONAL_DIR_NAME);
        let res = self
            .optional_addons
            .iter()
            .map(|part| optional_dir.join(part.get_name()).canonicalize())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(res)
    }
}

fn read_addons_to_part(folder: PathBuf, cache: &KVCache) -> anyhow::Result<Vec<PackPart>> {
    if !folder.is_dir() {
        anyhow::bail!("{} is not a directory", folder.display());
    }

    let entries = std::fs::read_dir(folder)?;
    let mut parts = vec![];

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let part = read_to_part(path, cache)?;
            parts.push(part);
        } else {
            eprintln!("{} is not a directory", path.display());
        }
    }

    parts.sort_by(|a, b| a.get_name().cmp(b.get_name()));

    Ok(parts)
}
