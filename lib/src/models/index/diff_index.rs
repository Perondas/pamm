use crate::models::index::index_node::{FileKind, IndexNode, NodeKind, PBOPart};
use crate::models::index::node_diff::{
    FileModification, ModifiedNodeKind, NodeDiff, NodeModification,
};
use crate::util::iterator_diff::{DiffResult, diff_iterators};
use rayon::prelude::*;

pub fn diff_index(old: &IndexNode, new: &IndexNode) -> anyhow::Result<NodeDiff> {
    let IndexNode {
        kind: old_kind,
        checksum: old_checksum,
        ..
    } = old;
    let IndexNode {
        kind: new_kind,
        checksum: new_checksum,
        name: new_name,
        ..
    } = new;

    if old_checksum == new_checksum {
        return Ok(NodeDiff::None(new_name.clone()));
    }

    let diff = match (old_kind, new_kind) {
        (NodeKind::File { .. }, NodeKind::File { .. }) => diff_file(old, new),
        (NodeKind::Folder(_), NodeKind::Folder(_)) => diff_folders(old, new)?,
        // On type mismatch we re-create everything as new
        (_, new_kind) => NodeDiff::Created(IndexNode {
            name: new_name.to_string(),
            checksum: new_checksum.clone(),
            kind: new_kind.clone(),
        }),
    };

    Ok(diff)
}

fn diff_file(old: &IndexNode, new: &IndexNode) -> NodeDiff {
    let (old_kind, old_length) = match &old.kind {
        NodeKind::File { kind, length } => (kind, *length),
        _ => unreachable!("diff_file called with non-File NodeKind on left"),
    };
    let (new_kind, new_length) = match &new.kind {
        NodeKind::File { kind, length } => (kind, *length),
        _ => unreachable!("diff_file called with non-File NodeKind on right"),
    };
    let new_checksum = &new.checksum;
    let new_name = &new.name;

    match (old_kind, new_kind) {
        (
            FileKind::Pbo {
                parts: old_parts, ..
            },
            FileKind::Pbo {
                parts: new_parts,
                blob_start,
                ..
            },
        ) => {
            let (required_checksums, required_parts_size) = diff_pbo_parts(old_parts, new_parts);
            NodeDiff::Modified(NodeModification {
                name: new_name.to_string(),
                kind: ModifiedNodeKind::File {
                    old_length,
                    target_checksum: new_checksum.to_owned(),
                    modification: FileModification::PBO {
                        new_length,
                        dl_size: required_parts_size + blob_start + 20,
                        required_checksums,
                        new_order: new_parts.clone(),
                        new_blob_start: *blob_start,
                    },
                },
            })
        }
        // In all other cases treat as generic file diff
        _ => NodeDiff::Modified(NodeModification {
            name: new_name.to_string(),
            kind: ModifiedNodeKind::File {
                old_length,
                target_checksum: new_checksum.to_owned(),
                modification: FileModification::Generic { new_length },
            },
        }),
    }
}

fn diff_pbo_parts(old_parts: &[PBOPart], new_parts: &[PBOPart]) -> (Vec<Vec<u8>>, u64) {
    let (required_parts_checksums, lengths): (Vec<_>, Vec<_>) = new_parts
        .iter()
        .filter(|p| !old_parts.iter().any(|o| o.checksum == p.checksum))
        .map(|p| (p.checksum.clone(), p.length))
        .unzip();

    let required_parts_size = lengths.iter().map(|&l| l as u64).sum();
    (required_parts_checksums, required_parts_size)
}

fn diff_folders(old_node: &IndexNode, new_node: &IndexNode) -> anyhow::Result<NodeDiff> {
    let old = match &old_node.kind {
        NodeKind::Folder(children) => children,
        _ => unreachable!("diff_folders called with non-Folder NodeKind on left"),
    };
    let new = match &new_node.kind {
        NodeKind::Folder(children) => children,
        _ => unreachable!("diff_folders called with non-Folder NodeKind on right"),
    };
    let new_name = &new_node.name;

    let all = diff_folder_contents(old, new)?;

    if all.is_empty() {
        Ok(NodeDiff::None(new_name.to_string()))
    } else {
        Ok(NodeDiff::Modified(NodeModification {
            name: new_name.to_string(),
            kind: ModifiedNodeKind::Folder(all),
        }))
    }
}

pub fn diff_folder_contents(old: &[IndexNode], new: &[IndexNode]) -> anyhow::Result<Vec<NodeDiff>> {
    let DiffResult {
        added,
        removed,
        same,
    } = diff_iterators(old, new, |node| &node.name);

    log::debug!(
        "Pack diff: {} added, {} removed, {} to check for modifications",
        added.len(),
        removed.len(),
        same.len()
    );

    let added = added
        .into_iter()
        .cloned()
        .map(NodeDiff::Created)
        .collect::<Vec<_>>();

    let removed = removed
        .into_iter()
        .map(|node| NodeDiff::Deleted(node.name.to_string()))
        .collect::<Vec<_>>();

    let changes = same
        .into_par_iter()
        .map(|(old, new)| diff_index(old, new))
        .collect::<Result<Vec<_>, _>>();

    Ok(added.into_iter().chain(removed).chain(changes?).collect())
}
