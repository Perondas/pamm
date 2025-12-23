use crate::index::index_node::{FileKind, IndexNode, NodeKind, PBOPart};
use crate::index::node_diff::{FileModification, ModifiedNodeKind, NodeDiff, NodeModification};
use std::collections::HashMap;

fn diff_index(left: IndexNode, right: IndexNode) -> anyhow::Result<NodeDiff> {
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
        return Ok(NodeDiff::None);
    }

    let diff = match (l_kind, r_kind) {
        (
            NodeKind::File { kind: l_kind, .. },
            NodeKind::File {
                kind: r_kind,
                length: r_length,
                ..
            },
        ) => diff_file(l_kind, r_kind, r_checksum, r_name, r_length),
        (NodeKind::Folder(left_children), NodeKind::Folder(right_children)) => {
            diff_folders(left_children, right_children, r_name)?
        }
        // On type mismatch we re-create everything as new
        (_, right_kind) => NodeDiff::Created(IndexNode {
            name: r_name,
            checksum: r_checksum,
            kind: right_kind,
        }),
    };

    Ok(diff)
}

fn diff_file(
    l_kind: FileKind,
    r_kind: FileKind,
    r_checksum: Vec<u8>,
    r_name: String,
    r_length: u64,
) -> NodeDiff {
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
            NodeDiff::Modified(NodeModification {
                name: r_name,
                kind: ModifiedNodeKind::File {
                    new_length: r_length,
                    target_checksum: r_checksum,
                    modification: FileModification::PBO {
                        required_parts_size,
                        required_checksums,
                        new_order: r_parts,
                        new_blob_offset: blob_offset,
                    },
                },
            })
        }
        // In all other cases treat as generic file diff
        _ => NodeDiff::Modified(NodeModification {
            name: r_name,
            kind: ModifiedNodeKind::File {
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

// Left is old right is new
fn diff_folders(
    left: Vec<IndexNode>,
    right: Vec<IndexNode>,
    r_name: String,
) -> anyhow::Result<NodeDiff> {
    let mut left_map = iter_to_map(left);

    let mut right_map = iter_to_map(right);

    // Ones that are in right but not left are added
    let added: Vec<_> = right_map
        .extract_if(|path, _| !left_map.contains_key(path))
        .map(|(_, part)| NodeDiff::Created(part))
        .collect();

    // Ones that are in the left but not right are deleted
    let removed: Vec<_> = left_map
        .extract_if(|path, _| !right_map.contains_key(path))
        .map(|(path, _)| NodeDiff::Deleted(path))
        .collect();

    let changes: Result<Vec<_>, _> = right_map
        .into_iter()
        .map(|(path, right_part)| (left_map.remove(&path).expect("Should exist"), right_part))
        .map(|(left, right)| diff_index(left, right))
        .collect();

    if !left_map.is_empty() {
        unreachable!("Left map should be empty after processing all right entries");
    }

    let all: Vec<NodeDiff> = added.into_iter().chain(removed).chain(changes?).collect();

    if all.is_empty() {
        Ok(NodeDiff::None)
    } else {
        Ok(NodeDiff::Modified(NodeModification {
            name: r_name,
            kind: ModifiedNodeKind::Folder(all),
        }))
    }
}

fn iter_to_map(entries: Vec<IndexNode>) -> HashMap<String, IndexNode> {
    entries
        .into_iter()
        .map(|e| (e.name.to_owned(), e))
        .collect()
}
