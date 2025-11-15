use crate::consts::{OPTIONAL_DIR_NAME, REQUIRED_DIR_NAME, STATE_DIR_NAME};
use crate::fs::part_reader::read_to_part;
use crate::pack::pack_diff::PackDiff;
use crate::pack::pack_manifest::PackManifest;
use crate::pack::pack_part::part::PackPart;
use anyhow::Result;
use std::path::{Path, PathBuf};

impl PackManifest {
    pub fn determine_pack_diff(&self, force_refresh: bool) -> Result<PackDiff> {
        let all_known_addons = &self.addons;

        let required_fs_addons = get_addons_in_folder(REQUIRED_DIR_NAME)?;
        let optional_fs_addons = get_addons_in_folder(OPTIONAL_DIR_NAME)?;
        let all_fs_addons: Vec<_> = required_fs_addons
            .into_iter()
            .chain(optional_fs_addons)
            .collect();

        let removed = all_known_addons
            .iter()
            .filter(|(path, _)| !all_fs_addons.contains(path))
            .map(|(path, _)| path)
            .cloned()
            .collect::<Vec<_>>();

        let created = all_fs_addons
            .iter()
            .filter(|addon| !all_known_addons.iter().any(|(path, _)| *addon == path))
            .cloned()
            .collect::<Vec<_>>();

        let potentially_modified = all_known_addons
            .iter()
            .filter(|(path, _)| all_fs_addons.contains(path))
            .cloned()
            .collect::<Vec<_>>();

        let mut added = Vec::with_capacity(created.len());

        for path in created {
            let created = read_to_part(PathBuf::from(&path), None)?;
            added.push((path, created));
        }

        let mut modified = Vec::new();

        for (path, _) in potentially_modified {
            let updated = update_part(&path, force_refresh)?;
            if let Some(updated) = updated {
                modified.push((path, updated));
            }
        }

        Ok(PackDiff {
            added,
            removed,
            modified,
        })
    }
}

fn get_addons_in_folder(dir_path: &str) -> Result<Vec<String>> {
    let mut addons = vec![];
    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir()
            && let Some(name) = path.file_name().and_then(|n| n.to_str())
        {
            let rel_path = format!("{}/{}", dir_path, name);
            addons.push(rel_path);
        }
    }
    Ok(addons)
}

fn update_part(path: &str, force_refresh: bool) -> Result<Option<PackPart>> {
    let path = PathBuf::from(path);

    let part_file = if force_refresh {
        None
    } else {
        try_load_part_file(&path)?
    };

    let new_part = read_to_part(path, part_file.as_ref())?;

    if let Some(part_file) = part_file {
        if part_file.get_checksum() == new_part.get_checksum() {
            Ok(None)
        } else {
            Ok(Some(new_part))
        }
    } else {
        Ok(Some(new_part))
    }
}

fn try_load_part_file(path: &Path) -> Result<Option<PackPart>> {
    let folder_name = path.file_name().unwrap().to_str().unwrap();

    let state_path = std::env::current_dir()?.join(STATE_DIR_NAME);
    let state_file_name = format!("{}.part", folder_name);
    let state_file_path = state_path.join(state_file_name);

    if !state_file_path.exists() {
        Ok(None)
    } else {
        let state_part: PackPart = serde_cbor::from_reader(std::fs::File::open(state_file_path)?)?;
        Ok(Some(state_part))
    }
}
