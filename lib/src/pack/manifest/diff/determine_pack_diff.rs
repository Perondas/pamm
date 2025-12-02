use crate::pack::manifest::diff::entry_diff::{
    EntryDiff, EntryModification, FileModification, ModifiedEntryKind,
};
use crate::pack::manifest::diff::pack_diff::PackDiff;
use crate::pack::manifest::entries::manifest_entry::{EntryKind, FileKind, ManifestEntry, PBOPart};
use crate::pack::manifest::pack_manifest::PackManifest;
use std::collections::HashMap;

impl PackManifest {
    pub fn determine_pack_diff(&self, other: &Self) -> anyhow::Result<PackDiff> {
        let required_diff = diff_folders(&self.required_addons, &other.required_addons);
        let optional_diff = diff_folders(&self.optional_addons, &other.optional_addons);

        Ok(PackDiff {
            required_changes: required_diff?,
            optional_changes: optional_diff?,
        })
    }
}

// Left is old right is new
fn diff_folders(left: &[ManifestEntry], right: &[ManifestEntry]) -> anyhow::Result<Vec<EntryDiff>> {
    let mut left_map: HashMap<String, ManifestEntry> = left
        .iter()
        .cloned()
        .map(|p| (p.name.to_owned(), p))
        .collect();

    let mut right_map: HashMap<String, ManifestEntry> = right
        .iter()
        .cloned()
        .map(|p| (p.name.to_owned(), p))
        .collect();

    // Ones that are in right but not left are added
    let added: Vec<_> = right_map
        .extract_if(|path, _| !left_map.contains_key(path))
        .map(|(_, part)| EntryDiff::Created(part))
        .collect();

    // Ones that are in the left but not right are deleted
    let removed: Vec<_> = left_map
        .extract_if(|path, _| !right_map.contains_key(path))
        .map(|(path, _)| EntryDiff::Deleted(path))
        .collect();

    let changes: Result<Vec<_>, _> = right_map
        .into_iter()
        .map(|(path, right_part)| (left_map.remove(&path).expect("Should exist"), right_part))
        .map(diff_entries)
        .collect();

    if !left_map.is_empty() {
        anyhow::bail!("Somehow we failed to match up a change")
    }

    Ok(added
        .into_iter()
        .chain(removed)
        .chain(changes?.into_iter().flatten())
        .collect())
}

fn diff_entries(pair: (ManifestEntry, ManifestEntry)) -> anyhow::Result<Option<EntryDiff>> {
    let (
        ManifestEntry {
            kind: l_kind,
            checksum: l_checksum,
            ..
        },
        ManifestEntry {
            kind: r_kind,
            checksum: r_checksum,
            name: r_name,
            ..
        },
    ) = pair;

    if l_checksum == r_checksum {
        return Ok(None);
    }

    let diff = match (l_kind, r_kind) {
        (
            EntryKind::File { kind: l_kind, .. },
            EntryKind::File {
                kind: r_kind,
                length: r_length,
                ..
            },
        ) => diff_file(l_kind, r_kind, r_checksum, r_name, r_length),
        (EntryKind::Folder(left_children), EntryKind::Folder(right_children)) => {
            let diff = diff_folders(&left_children, &right_children)?;
            if diff.is_empty() {
                return Ok(None);
            }
            EntryDiff::Modified(EntryModification {
                name: r_name,
                kind: ModifiedEntryKind::Folder(diff),
            })
        }
        // On type mismatch we re-create everything as new
        (_, right_kind) => EntryDiff::Created(ManifestEntry {
            name: r_name,
            checksum: r_checksum,
            kind: right_kind,
        }),
    };

    Ok(Some(diff))
}

fn diff_file(
    l_kind: FileKind,
    r_kind: FileKind,
    r_checksum: Vec<u8>,
    r_name: String,
    r_length: u64,
) -> EntryDiff {
    match (l_kind, r_kind) {
        (
            FileKind::Pbo { parts: l_parts, .. },
            FileKind::Pbo {
                parts: r_parts,
                blob_offset,
                ..
            },
        ) => {
            let (required_checksums, required_parts_size) = diff_pbo_parts(&l_parts, &r_parts);
            EntryDiff::Modified(EntryModification {
                name: r_name,
                kind: ModifiedEntryKind::File {
                    new_length: r_length,
                    target_checksum: r_checksum,
                    modification: FileModification::PBO {
                        required_parts_size,
                        required_checksums,
                        new_order: r_parts,
                        blob_offset,
                        new_length: r_length,
                    },
                },
            })
        }
        // In all other cases treat as generic file diff
        _ => EntryDiff::Modified(EntryModification {
            name: r_name,
            kind: ModifiedEntryKind::File {
                new_length: r_length,
                target_checksum: r_checksum,
                modification: FileModification::Generic,
            },
        }),
    }
}

fn diff_pbo_parts(left_parts: &[PBOPart], right_parts: &[PBOPart]) -> (Vec<Vec<u8>>, u64) {
    let (required_parts_checksums, lengths): (Vec<_>, Vec<_>) = right_parts
        .iter()
        .filter(|p| !left_parts.iter().any(|l| l.checksum == p.checksum))
        .map(|p| (p.checksum.clone(), p.length))
        .unzip();

    let required_parts_size = lengths.iter().map(|&l| l as u64).sum();
    (required_parts_checksums, required_parts_size)
}
