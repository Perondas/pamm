use crate::manifest::diff::entry_diff::{
    EntryDiff, EntryModification, FileModification, ModifiedEntryKind,
};
use crate::manifest::diff::pack_diff::PackDiff;
use crate::manifest::entries::manifest_entry::{EntryKind, ManifestEntry};
use crate::name_consts::get_pack_addon_directory_name;
use crate::net::patcher::{download_file, patch_pbo_file};
use anyhow::Context;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::fs;
use std::path::Path;
use url::Url;

pub fn apply_diff(
    pack_path: &Path,
    name: &str,
    pack_diff: PackDiff,
    base_url: &Url,
) -> anyhow::Result<()> {
    let PackDiff { changes } = pack_diff;
    let addon_dir_name = get_pack_addon_directory_name(name);
    let addons_path = pack_path.join(&addon_dir_name);
    let addons_url = base_url.join(&format!("{addon_dir_name}/"))?;
    patch_dir(&changes, &addons_path, &addons_url)?;

    Ok(())
}

fn patch_dir(diffs: &[EntryDiff], dir_path: &Path, url: &Url) -> anyhow::Result<()> {
    let res = diffs
        .into_par_iter()
        .map(|modification| match modification {
            EntryDiff::Created(entry) => create_entry(entry, dir_path, url),
            EntryDiff::Deleted(path) => {
                let full_path = dir_path.join(path);
                if full_path.is_dir() {
                    fs::remove_dir_all(&full_path).context("Failed to delete directory")
                } else if full_path.is_file() {
                    fs::remove_file(&full_path).context("Failed to delete file")
                } else {
                    unreachable!(
                        "Path to delete is neither file nor directory: {:?}",
                        full_path
                    );
                }
            }
            EntryDiff::Modified(modification) => apply_modification(modification, dir_path, url),
        })
        .filter_map(|res| res.err())
        .collect::<Vec<_>>();

    if !res.is_empty() {
        let combined_error = res.into_iter().fold(
            anyhow::anyhow!("One or more errors occurred while applying diff"),
            |acc, err| acc.context(err),
        );
        return Err(combined_error);
    }

    Ok(())
}

fn create_entry(entry: &ManifestEntry, parent_path: &Path, parent_url: &Url) -> anyhow::Result<()> {
    let url = join_to_url(parent_url, &entry.name)?;
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
    let url = join_to_url(parent_url, &modification.name)?;
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
                    &url,
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

fn join_to_url(base: &Url, segment: &str) -> anyhow::Result<Url> {
    // Annoyingly the url crate does not have a method to join a path segment without
    // interpreting it as a file name, so we add a trailing slash to force it to be treated
    // as a directory segment.

    base.join(&format!("{}/", segment)).context(format!(
        "Failed to create url by joining {} with {}",
        base, segment
    ))
}
