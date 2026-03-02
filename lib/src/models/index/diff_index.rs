use crate::models::index::index_node::{FileKind, IndexNode, NodeKind, PBOPart};
use crate::models::index::node_diff::{
    FileModification, ModifiedNodeKind, NodeDiff, NodeModification,
};
use crate::util::iterator_diff::{diff_iterators, DiffResult};
use rayon::prelude::*;

pub fn diff_index(left: &IndexNode, right: &IndexNode) -> anyhow::Result<NodeDiff> {
    let IndexNode {
        kind: l_kind,
        checksum: l_checksum,
        ..
    } = left;
    let IndexNode {
        kind: r_kind,
        checksum: r_checksum,
        name: r_name,
        ..
    } = right;

    if l_checksum == r_checksum {
        return Ok(NodeDiff::None(r_name.clone()));
    }

    let diff = match (l_kind, r_kind) {
        (
            NodeKind::File {
                kind: l_kind,
                length: l_length,
                ..
            },
            NodeKind::File {
                kind: r_kind,
                length: r_length,
                ..
            },
        ) => diff_file(l_kind, r_kind, r_checksum, r_name, *l_length, *r_length),
        (NodeKind::Folder(left_children), NodeKind::Folder(right_children)) => {
            diff_folders(left_children, right_children, r_name)?
        }
        // On type mismatch we re-create everything as new
        (_, right_kind) => NodeDiff::Created(IndexNode {
            name: r_name.to_string(),
            checksum: r_checksum.clone(),
            kind: right_kind.clone(),
        }),
    };

    Ok(diff)
}

fn diff_file(
    l_kind: &FileKind,
    r_kind: &FileKind,
    r_checksum: &Vec<u8>,
    r_name: &str,
    l_length: u64,
    r_length: u64,
) -> NodeDiff {
    match (l_kind, r_kind) {
        (
            FileKind::Pbo { parts: l_parts, .. },
            FileKind::Pbo {
                parts: r_parts,
                blob_start,
                ..
            },
        ) => {
            let (required_checksums, required_parts_size) = diff_pbo_parts(&l_parts, &r_parts);
            NodeDiff::Modified(NodeModification {
                name: r_name.to_string(),
                kind: ModifiedNodeKind::File {
                    old_length: l_length,
                    target_checksum: r_checksum.clone(),
                    modification: FileModification::PBO {
                        new_length: r_length,
                        dl_size: required_parts_size + blob_start + 20,
                        required_checksums,
                        new_order: r_parts.clone(),
                        new_blob_start: *blob_start,
                    },
                },
            })
        }
        // In all other cases treat as generic file diff
        _ => NodeDiff::Modified(NodeModification {
            name: r_name.to_string(),
            kind: ModifiedNodeKind::File {
                old_length: l_length,
                target_checksum: r_checksum.clone(),
                modification: FileModification::Generic {
                    new_length: r_length,
                },
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

// Left is old right is new
fn diff_folders(
    old: &Vec<IndexNode>,
    new: &Vec<IndexNode>,
    r_name: &str,
) -> anyhow::Result<NodeDiff> {
    let DiffResult {
        added,
        removed,
        same,
    } = diff_iterators(old, new, |node| node.name.clone());

    let added = added
        .into_iter()
        .cloned()
        .map(NodeDiff::Created)
        .collect::<Vec<_>>();

    let removed = removed
        .into_iter()
        .map(|node| NodeDiff::Deleted(node.name.to_string()))
        .collect::<Vec<_>>();

    let changes: Result<Vec<_>, _> = same
        .into_par_iter()
        .map(|(left, right)| diff_index(left, right))
        .collect();

    let all: Vec<NodeDiff> = added.into_iter().chain(removed).chain(changes?).collect();

    if all.is_empty() {
        Ok(NodeDiff::None(r_name.to_string()))
    } else {
        Ok(NodeDiff::Modified(NodeModification {
            name: r_name.to_string(),
            kind: ModifiedNodeKind::Folder(all),
        }))
    }
}
