use crate::consts::{OPTIONAL_DIR_NAME, REQUIRED_DIR_NAME};
use crate::net::patcher::{download_file, patch_generic_file, patch_pbo_file};
use crate::pack::pack_diff::PackDiff;
use crate::pack::pack_part::folder::Folder;
use crate::pack::pack_part::part::{File, PackPart};
use crate::pack::part_diff::{FileModification, PartDiff, PartModification};
use std::fs;
use std::path::Path;
use url::Url;

pub fn apply_diff(pack_path: &Path, pack_diff: PackDiff, base_url: &Url) -> anyhow::Result<()> {
    let PackDiff {
        required_changes,
        optional_changes,
    } = pack_diff;

    let required_path = pack_path.join(REQUIRED_DIR_NAME);
    let required_url = base_url.join(&format!("{REQUIRED_DIR_NAME}/"))?;
    patch_dir(&required_changes, &required_path, &required_url)?;

    let optional_path = pack_path.join(OPTIONAL_DIR_NAME);
    let optional_url = base_url.join(&format!("{OPTIONAL_DIR_NAME}/"))?;
    patch_dir(&optional_changes, &optional_path, &optional_url)?;

    Ok(())
}

fn patch_dir(diffs: &[PartDiff], destination_path: &Path, url: &Url) -> anyhow::Result<()> {
    for modification in diffs {
        match modification {
            PartDiff::Created(part) => match part {
                PackPart::Folder(f) => {
                    dl_folder_recursively(destination_path, f, url)?;
                }
                PackPart::File(f) => {
                    dl_file(destination_path, f, url)?;
                }
            },
            PartDiff::Deleted(path) => {
                let full_path = destination_path.join(path);
                if full_path.is_dir() {
                    // TODO: check that this is sensible
                    //fs::remove_dir_all(&full_path)?;
                } else if full_path.is_file() {
                    fs::remove_file(&full_path)?;
                }
            }
            PartDiff::Modified(modification) => match modification {
                PartModification::Folder(f) => {
                    let new_path = destination_path.join(&f.name);
                    let new_url = url.join(&format!("{}/", &f.name))?;
                    patch_dir(&f.changes, &new_path, &new_url)?;
                }
                PartModification::File(f) => match f {
                    FileModification::PBO(p) => {
                        let file_path = destination_path.join(&p.name);
                        let file_url = url.join(&p.name)?;
                        patch_pbo_file(&file_path, file_url, p)?;
                    }
                    FileModification::GenericFile(g) => {
                        let file_path = destination_path.join(&g.name);
                        let file_url = url.join(&g.name)?;
                        patch_generic_file(&file_path, file_url, g)?;
                    }
                },
            },
        }
    }
    Ok(())
}

fn dl_folder_recursively(path: &Path, folder: &Folder, url: &Url) -> anyhow::Result<()> {
    let folder_path = path.join(&folder.name);
    let folder_url = url.join(&format!("{}/", &folder.name))?;

    fs::create_dir_all(&folder_path)?;

    for child in folder.children.iter() {
        match child {
            PackPart::Folder(f) => {
                dl_folder_recursively(&folder_path, f, &folder_url)?;
            }
            PackPart::File(f) => {
                dl_file(&folder_path, f, &folder_url)?;
            }
        }
    }
    Ok(())
}

fn dl_file(path: &Path, file: &File, url: &Url) -> anyhow::Result<()> {
    let file_path = path.join(file.get_name());
    let file_url = url.join(file.get_name())?;
    download_file(&file_path, file_url)?;
    Ok(())
}
