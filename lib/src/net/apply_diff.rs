use crate::name_consts::{OPTIONAL_DIR_NAME, REQUIRED_DIR_NAME};
use crate::net::patcher::{download_file, patch_pbo_file};
use crate::pack::manifest::diff::entry_diff::{
    EntryDiff, EntryModification, FileModification, ModifiedEntryKind,
};
use crate::pack::manifest::diff::pack_diff::PackDiff;
use crate::pack::manifest::entries::manifest_entry::{EntryKind, ManifestEntry};
use anyhow::Context;
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

fn patch_dir(diffs: &[EntryDiff], dir_path: &Path, url: &Url) -> anyhow::Result<()> {
    for modification in diffs {
        match modification {
            EntryDiff::Created(entry) => create_entry(entry, dir_path, url)?,
            EntryDiff::Deleted(path) => {
                let full_path = dir_path.join(path);
                if full_path.is_dir() {
                    // TODO: check that this is sensible
                    //fs::remove_dir_all(&full_path)?;
                } else if full_path.is_file() {
                    fs::remove_file(&full_path)?;
                }
            }
            EntryDiff::Modified(modification) => apply_modification(modification, dir_path, url)?,
        }
    }

    Ok(())
}

fn create_entry(entry: &ManifestEntry, parent_path: &Path, parent_url: &Url) -> anyhow::Result<()> {
    let url = join_url(parent_url, &entry.name)?;
    let path = parent_path.join(&entry.name);

    match &entry.kind {
        EntryKind::Folder(children) => {
            fs::create_dir_all(&path).context("Failed to create folder")?;
            for child in children {
                create_entry(child, &path, &url)?;
            }
        }
        EntryKind::File { .. } => {
            download_file(&path, &url)?;
        }
    }

    Ok(())
}

fn apply_modification(
    modification: &EntryModification,
    parent_path: &Path,
    parent_url: &Url,
) -> anyhow::Result<()> {
    let url = join_url(parent_url, &modification.name)?;
    let path = parent_path.join(&modification.name);

    match &modification.kind {
        ModifiedEntryKind::Folder(children) => patch_dir(children, &path, &url)?,
        ModifiedEntryKind::File { modification, .. } => {
            match modification {
                FileModification::PBO {
                    new_order,
                    required_checksums,
                    new_length,
                    blob_offset,
                    ..
                } => patch_pbo_file(
                    &path,
                    url,
                    new_order,
                    required_checksums,
                    *new_length,
                    *blob_offset,
                )?,
                // Just re-download the file for generic modifications
                FileModification::Generic => download_file(&path, &url)?,
            }
        }
    }

    Ok(())
}

fn join_url(base: &Url, segment: &str) -> anyhow::Result<Url> {
    base.join(&format!("/{}", segment)).context(format!(
        "Failed to create url by joining {} with {}",
        base, segment
    ))
}
