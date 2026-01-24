use flutter_rust_bridge::frb;
use index::node_diff;
use node_diff::{FileModification, ModifiedNodeKind};
use pamm_lib::index;
use pamm_lib::index::index_node::{IndexNode, NodeKind};
use pamm_lib::index::node_diff::{NodeDiff, NodeModification};

#[derive(Clone)]
pub struct FileChange {
    pub file_path: String,
    pub change: ChangeType,
}

#[derive(Clone)]
pub enum ChangeType {
    Created { size: u64 },
    Deleted,
    Modified { size_change: i64, dl_size: u64 },
}

#[frb(ignore)]
pub fn get_file_changes(diff: &NodeDiff) -> Vec<FileChange> {
    let mut changes = Vec::new();

    match diff {
        NodeDiff::Created(node) => {
            changes.extend(collect_created_files(node));
        }
        NodeDiff::Deleted(path) => {
            changes.push(FileChange {
                file_path: path.clone(),
                change: ChangeType::Deleted,
            });
        }
        NodeDiff::Modified(modification) => {
            changes.extend(collect_modified_files(modification));
        }
        NodeDiff::None(_) => {}
    }

    changes
}

fn collect_modified_files(node: &NodeModification) -> Vec<FileChange> {
    let mut changes = Vec::new();

    match &node.kind {
        ModifiedNodeKind::Folder(children) => {
            for child in children {
                changes.extend(get_file_changes(child));
            }
        }
        ModifiedNodeKind::File {
            old_length,
            modification,
            ..
        } => {
            let (size_change, dl_size) = match modification {
                FileModification::PBO {
                    new_length,
                    required_parts_size,
                    ..
                } => (
                    *new_length as i64 - *old_length as i64,
                    *required_parts_size,
                ),
                FileModification::Generic { new_length } => (
                    *new_length as i64 - *old_length as i64,
                    (*new_length).saturating_sub(*old_length),
                ),
            };

            changes.push(FileChange {
                file_path: node.name.clone(),
                change: ChangeType::Modified {
                    size_change,
                    dl_size,
                },
            });
        }
    }

    changes
}

fn collect_created_files(node: &IndexNode) -> Vec<FileChange> {
    let mut changes = Vec::new();

    match &node.kind {
        NodeKind::Folder(children) => {
            for child in children {
                changes.extend(collect_created_files(child));
            }
        }
        NodeKind::File { length, .. } => {
            changes.push(FileChange {
                file_path: node.name.clone(),
                change: ChangeType::Created { size: *length },
            });
        }
    }

    changes
}
