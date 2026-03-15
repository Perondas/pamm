use crate::models::index::index_node::{IndexNode, NodeKind};
use crate::models::index::node_diff::{ModifiedNodeKind, NodeDiff};

pub trait GetSizeChange {
    fn get_size_change(&self) -> i64;
}

impl GetSizeChange for IndexNode {
    fn get_size_change(&self) -> i64 {
        match &self.kind {
            NodeKind::File { length, .. } => *length as i64,
            NodeKind::Folder(children) => {
                children.iter().map(|child| child.get_size_change()).sum()
            }
        }
    }
}

impl GetSizeChange for NodeDiff {
    fn get_size_change(&self) -> i64 {
        match self {
            NodeDiff::Created(entry) => entry.get_size_change(),
            NodeDiff::Modified(modification) => match &modification.kind {
                ModifiedNodeKind::Folder(e) => e.iter().map(|child| child.get_size_change()).sum(),
                ModifiedNodeKind::File {
                    modification,
                    old_length,
                    ..
                } => modification.get_length() as i64 - *old_length as i64,
            },
            NodeDiff::Deleted { size, .. } => -(*size as i64),
            NodeDiff::None(_) => 0,
        }
    }
}
