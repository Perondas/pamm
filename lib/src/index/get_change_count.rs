use crate::index::index_node::{IndexNode, NodeKind};
use crate::index::node_diff::{ModifiedNodeKind, NodeDiff, NodeModification};

// TODO: maybe remove this trait
#[allow(dead_code)]
pub trait GetChangeCount {
    fn get_change_count(&self) -> u64;
}

impl GetChangeCount for IndexNode {
    fn get_change_count(&self) -> u64 {
        match &self.kind {
            NodeKind::Folder(children) => {
                children.iter().map(|child| child.get_change_count()).sum()
            }
            NodeKind::File { .. } => 1,
        }
    }
}

impl GetChangeCount for NodeModification {
    fn get_change_count(&self) -> u64 {
        match &self.kind {
            ModifiedNodeKind::Folder(children) => {
                children.iter().map(|child| child.get_change_count()).sum()
            }
            ModifiedNodeKind::File { .. } => 1,
        }
    }
}

impl GetChangeCount for NodeDiff {
    fn get_change_count(&self) -> u64 {
        match self {
            NodeDiff::Created(node) => node.get_change_count(),
            NodeDiff::Deleted(_) => 1,
            NodeDiff::Modified(modification) => modification.get_change_count(),
            NodeDiff::None(_) => 0,
        }
    }
}
