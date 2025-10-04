use crate::consts::{OPTIONAL_DIR_NAME, REQUIRED_DIR_NAME, STATE_DIR_NAME};
use crate::fs::part_reader::read_to_part;
use crate::pack::pack_diff::PackDiff;
use crate::pack::pack_manifest::PackManifest;
use crate::pack::pack_part::part::PackPart;
use anyhow::Result;
use std::path::PathBuf;

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
            .filter(|addon| !all_fs_addons.contains(addon))
            .cloned()
            .collect::<Vec<_>>();

        let created = all_fs_addons
            .iter()
            .filter(|addon| !all_known_addons.contains(addon))
            .cloned()
            .collect::<Vec<_>>();

        let potentially_modified = all_known_addons
            .iter()
            .filter(|addon| all_fs_addons.contains(addon))
            .cloned()
            .collect::<Vec<_>>();

        let created_parts = created
            .iter()
            .map(|path| index_creation(path))
            .collect::<Result<Vec<_>>>()?;

        let modified_parts = potentially_modified
            .iter()
            .map(|path| index_update(path, force_refresh))
            .collect::<Result<Vec<_>>>()?;

        Ok(PackDiff {
            added: created.into_iter().zip(created_parts).collect(),
            removed,
            changed: potentially_modified
                .into_iter()
                .zip(modified_parts)
                .filter_map(|(path, part_opt)| part_opt.map(|part| (path, part)))
                .collect(),
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

fn index_creation(path: &str) -> Result<PackPart> {
    read_to_part(PathBuf::from(path), None)
}

fn index_update(path: &str, force_refresh: bool) -> Result<Option<PackPart>> {
    let path = PathBuf::from(path);
    let folder_name = path.file_name().unwrap().to_str().unwrap();

    let state_path = std::env::current_dir()?.join(STATE_DIR_NAME);
    let state_file_name = format!("{}.part", folder_name);
    let state_file_path = state_path.join(state_file_name);
    if !state_file_path.exists() || force_refresh {
        Ok(Some(read_to_part(path, None)?))
    } else {
        let state_part: PackPart = serde_cbor::from_reader(std::fs::File::open(state_file_path)?)?;
        let stored_checksum = state_part.get_checksum();
        let new_part = read_to_part(path, Some(state_part))?;
        if stored_checksum == new_part.get_checksum() {
            Ok(None)
        } else {
            Ok(Some(new_part))
        }
    }
}
