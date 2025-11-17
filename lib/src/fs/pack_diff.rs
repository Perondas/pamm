use crate::pack::pack_diff::PackDiff;
use crate::pack::pack_manifest::PackManifest;
use crate::pack::pack_part::folder::Folder;
use crate::pack::pack_part::part::{File, PackPart};
use crate::pack::part_diff::FileModification::{GenericFile, PBO};
use crate::pack::part_diff::{
    FolderModification, GenericFileModification, PBOModification, PartDiff, PartModification,
};
use std::collections::HashMap;

impl PackManifest {
    pub fn determine_pack_diff(&self, other: &Self) -> anyhow::Result<PackDiff> {
        let required_diff = diff_parts(&self.required_addons, &other.required_addons);
        let optional_diff = diff_parts(&self.optional_addons, &other.optional_addons);

        Ok(PackDiff {
            required_changes: required_diff?,
            optional_changes: optional_diff?,
        })
    }
}

// Left is old right is new
fn diff_parts(left: &[PackPart], right: &[PackPart]) -> anyhow::Result<Vec<PartDiff>> {
    let mut left_map: HashMap<String, PackPart> = left
        .iter()
        .cloned()
        .map(|p| (p.get_name().to_owned(), p))
        .collect();

    let mut right_map: HashMap<String, PackPart> = right
        .iter()
        .cloned()
        .map(|p| (p.get_name().to_owned(), p))
        .collect();

    // Ones that are in right but not left are added
    let added: Vec<_> = right_map
        .extract_if(|path, _| !left_map.contains_key(path))
        .map(|(_, part)| PartDiff::Created(part))
        .collect();

    // Ones that are in the left but not right are deleted
    let removed: Vec<_> = left_map
        .extract_if(|path, _| !right_map.contains_key(path))
        .map(|(path, _)| PartDiff::Deleted(path))
        .collect();

    let changes: Result<Vec<_>, _> = right_map
        .into_iter()
        .map(|(path, right_part)| (left_map.remove(&path).expect("Should exist"), right_part))
        .map(diff_part)
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

fn diff_part(pair: (PackPart, PackPart)) -> anyhow::Result<Option<PartDiff>> {
    if pair.0.get_checksum() == pair.1.get_checksum() {
        return Ok(None);
    }

    Ok(Some(match pair {
        (PackPart::File(left_file), PackPart::File(right_file)) => {
            diff_file(left_file, right_file)?
        }
        (PackPart::Folder(left_folder), PackPart::Folder(right_folder)) => {
            diff_folder(left_folder, right_folder)?
        }
        // On type mismatch we re-create everything as new
        (_, right) => PartDiff::Created(right),
    }))
}

fn diff_file(left: File, right: File) -> anyhow::Result<PartDiff> {
    match (left, right) {
        (File::Generic(_), File::Generic(right_file)) => Ok(PartDiff::Modified(
            PartModification::File(GenericFile(GenericFileModification {
                name: right_file.name,
                target_checksum: right_file.checksum,
                new_length: right_file.length,
            })),
        )),
        (File::PBO(left_file), File::PBO(right_file)) => {
            let required_parts = right_file
                .parts
                .iter()
                .filter(|p| !left_file.parts.iter().any(|l| l.checksum == p.checksum))
                .map(|p| p.checksum.clone())
                .collect();

            Ok(PartDiff::Modified(PartModification::File(PBO(
                PBOModification {
                    name: right_file.name,
                    target_checksum: right_file.checksum,
                    new_order: right_file.parts,
                    required_parts,
                    blob_offset: right_file.blob_offset,
                },
            ))))
        }
        // On type mismatch we re-create everything as new
        (_, right) => Ok(PartDiff::Created(PackPart::File(right))),
    }
}

fn diff_folder(left: Folder, right: Folder) -> anyhow::Result<PartDiff> {
    let changes = diff_parts(&left.children, &right.children)?;
    Ok(PartDiff::Modified(PartModification::Folder(
        FolderModification {
            name: right.name,
            changes,
        },
    )))
}
