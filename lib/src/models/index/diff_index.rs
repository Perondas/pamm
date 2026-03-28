use crate::models::index::index_node::{FileKind, IndexNode, NodeKind, PBOPart};
use crate::models::index::node_diff::{
    FileModification, ModifiedNodeKind, NodeDiff, NodeModification,
};
use crate::util::iterator_diff::{DiffResult, diff_iterators};
use rayon::prelude::*;
use crate::models::index::get_size_change::GetSizeChange;

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
        .map(|node| NodeDiff::Deleted {
            name: node.name.to_string(),
            size: node.get_size_change() as u64,
        })
        .collect::<Vec<_>>();

    let changes = same
        .into_par_iter()
        .map(|(old, new)| diff_index(old, new))
        .collect::<Result<Vec<_>, _>>();

    Ok(added.into_iter().chain(removed).chain(changes?).collect())
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::models::index::index_node::{FileKind, IndexNode, NodeKind};
    use crate::models::index::node_diff::{FileModification, ModifiedNodeKind, NodeDiff};

    #[test]
    fn test_diff_same_checksum() {
        let node = IndexNode {
            name: "test".to_string(),
            checksum: vec![1, 2, 3],
            kind: NodeKind::File {
                length: 10,
                kind: FileKind::Generic,
            },
        };
        let diff = diff_index(&node, &node).unwrap();
        assert!(matches!(diff, NodeDiff::None(name) if name == "test"));
    }

    #[test]
    fn test_diff_generic_file_modified() {
        let old = IndexNode {
            name: "test".to_string(),
            checksum: vec![1, 2, 3],
            kind: NodeKind::File {
                length: 10,
                kind: FileKind::Generic,
            },
        };
        let new = IndexNode {
            name: "test".to_string(),
            checksum: vec![4, 5, 6],
            kind: NodeKind::File {
                length: 20,
                kind: FileKind::Generic,
            },
        };
        let diff = diff_index(&old, &new).unwrap();
        if let NodeDiff::Modified(mod_node) = diff {
            assert_eq!(mod_node.name, "test");
            match mod_node.kind {
                ModifiedNodeKind::File { old_length, target_checksum, modification } => {
                    assert_eq!(old_length, 10);
                    assert_eq!(target_checksum, vec![4, 5, 6]);
                    if let FileModification::Generic { new_length } = modification {
                        assert_eq!(new_length, 20);
                    } else {
                        panic!("Expected generic modification");
                    }
                }
                _ => panic!("Expected file modification"),
            }
        } else {
            panic!("Expected NodeDiff::Modified");
        }
    }

    #[test]
    fn test_diff_different_type() {
        let old = IndexNode {
            name: "test".to_string(),
            checksum: vec![1, 2, 3],
            kind: NodeKind::File {
                length: 10,
                kind: FileKind::Generic,
            },
        };
        let new = IndexNode {
            name: "test".to_string(),
            checksum: vec![4, 5, 6],
            kind: NodeKind::Folder(vec![]),
        };
        let diff = diff_index(&old, &new).unwrap();
        if let NodeDiff::Created(node) = diff {
            assert_eq!(node.name, "test");
            assert_eq!(node.checksum, vec![4, 5, 6]);
            assert!(matches!(node.kind, NodeKind::Folder(_)));
        } else {
            panic!("Expected NodeDiff::Created");
        }
    }

    #[test]
    fn test_diff_pbo_file_modified() {
        use crate::models::index::index_node::PBOPart;

        let old_parts = vec![
            PBOPart {
                name: "part1".to_string(),
                length: 100,
                checksum: vec![10],
                start_offset: 0,
            },
            PBOPart {
                name: "part2".to_string(),
                length: 200,
                checksum: vec![20],
                start_offset: 100,
            },
        ];

        let new_parts = vec![
            PBOPart {
                name: "part1".to_string(),
                length: 100,
                checksum: vec![10],
                start_offset: 0,
            },
            PBOPart {
                name: "part3".to_string(),
                length: 300,
                checksum: vec![30],
                start_offset: 100,
            },
        ];

        let old = IndexNode {
            name: "test.pbo".to_string(),
            checksum: vec![1, 1, 1],
            kind: NodeKind::File {
                length: 300,
                kind: FileKind::Pbo {
                    blob_start: 50,
                    parts: old_parts.clone(),
                },
            },
        };

        let new = IndexNode {
            name: "test.pbo".to_string(),
            checksum: vec![2, 2, 2],
            kind: NodeKind::File {
                length: 400,
                kind: FileKind::Pbo {
                    blob_start: 60,
                    parts: new_parts.clone(),
                },
            },
        };

        let diff = diff_index(&old, &new).unwrap();
        if let NodeDiff::Modified(mod_node) = diff {
            assert_eq!(mod_node.name, "test.pbo");
            match mod_node.kind {
                ModifiedNodeKind::File { old_length, target_checksum, modification } => {
                    assert_eq!(old_length, 300);
                    assert_eq!(target_checksum, vec![2, 2, 2]);
                    if let FileModification::PBO { new_length, dl_size, required_checksums, new_order, new_blob_start } = modification {
                        assert_eq!(new_length, 400);
                        // required_parts_size for part3 (300) + blob_start (60) + 20 = 380
                        assert_eq!(dl_size, 380);
                        assert_eq!(required_checksums, vec![vec![30]]);
                        assert_eq!(new_order.len(), 2);
                        assert_eq!(new_blob_start, 60);
                    } else {
                        panic!("Expected PBO modification");
                    }
                }
                _ => panic!("Expected file modification"),
            }
        } else {
            panic!("Expected NodeDiff::Modified");
        }
    }

    #[test]
    fn test_diff_folders_same() {
        let child1 = IndexNode {
            name: "file.txt".to_string(),
            checksum: vec![1],
            kind: NodeKind::File { length: 5, kind: FileKind::Generic },
        };
        let old = IndexNode {
            name: "dir".to_string(),
            checksum: vec![100],
            kind: NodeKind::Folder(vec![child1.clone()]),
        };
        let new = IndexNode {
            name: "dir".to_string(),
            checksum: vec![100],
            kind: NodeKind::Folder(vec![child1]),
        };
        let diff = diff_index(&old, &new).unwrap();
        assert!(matches!(diff, NodeDiff::None(name) if name == "dir"));
    }

    #[test]
    fn test_diff_folders_modified() {
        let child1 = IndexNode {
            name: "file1.txt".to_string(),
            checksum: vec![1],
            kind: NodeKind::File { length: 5, kind: FileKind::Generic },
        };
        let child2 = IndexNode {
            name: "file2.txt".to_string(),
            checksum: vec![2],
            kind: NodeKind::File { length: 10, kind: FileKind::Generic },
        };
        let old = IndexNode {
            name: "dir".to_string(),
            checksum: vec![100],
            kind: NodeKind::Folder(vec![child1.clone()]),
        };
        let new = IndexNode {
            name: "dir".to_string(),
            checksum: vec![200], // changed checksum
            kind: NodeKind::Folder(vec![child1, child2.clone()]),
        };
        let diff = diff_index(&old, &new).unwrap();
        if let NodeDiff::Modified(mod_node) = diff {
            assert_eq!(mod_node.name, "dir");
            if let ModifiedNodeKind::Folder(children_diff) = mod_node.kind {
                // There might be a DiffResult with Same and Added
                assert_eq!(children_diff.len(), 2);

                // One should be None (file1.txt), other should be Created (file2.txt)
                let mut created_found = false;
                for child_diff in children_diff {
                    match child_diff {
                        NodeDiff::Created(node) => {
                            assert_eq!(node.name, "file2.txt");
                            created_found = true;
                        },
                        NodeDiff::None(name) => {
                            assert_eq!(name, "file1.txt");
                        },
                        _ => panic!("Unexpected diff kind"),
                    }
                }
                assert!(created_found, "NodeDiff::Created not found");
            } else {
                panic!("Expected ModifiedNodeKind::Folder");
            }
        } else {
            panic!("Expected NodeDiff::Modified");
        }
    }

    #[test]
    #[should_panic(expected = "diff_folders called with non-Folder NodeKind on left")]
    fn test_diff_folders_panic_left() {
        let old = IndexNode {
            name: "dir".to_string(),
            checksum: vec![100],
            kind: NodeKind::File { length: 5, kind: FileKind::Generic },
        };
        let new = IndexNode {
            name: "dir".to_string(),
            checksum: vec![200],
            kind: NodeKind::Folder(vec![]),
        };
        let _ = diff_folders(&old, &new);
    }

    #[test]
    #[should_panic(expected = "diff_folders called with non-Folder NodeKind on right")]
    fn test_diff_folders_panic_right() {
        let old = IndexNode {
            name: "dir".to_string(),
            checksum: vec![100],
            kind: NodeKind::Folder(vec![]),
        };
        let new = IndexNode {
            name: "dir".to_string(),
            checksum: vec![200],
            kind: NodeKind::File { length: 5, kind: FileKind::Generic },
        };
        let _ = diff_folders(&old, &new);
    }
}