use crate::index::index_node::{IndexNode, NodeKind};
use crate::index::node_diff::{FileModification, ModifiedNodeKind, NodeDiff};

pub trait GetSize {
    fn get_size(&self) -> u64;
}

impl GetSize for IndexNode {
    fn get_size(&self) -> u64 {
        match &self.kind {
            NodeKind::File { length, .. } => *length,
            NodeKind::Folder(children) => children.iter().map(|child| child.get_size()).sum(),
        }
    }
}

impl GetSize for NodeDiff {
    fn get_size(&self) -> u64 {
        match self {
            NodeDiff::Created(entry) => entry.get_size(),
            NodeDiff::Modified(modification) => match &modification.kind {
                ModifiedNodeKind::Folder(e) => e.iter().map(|child| child.get_size()).sum(),
                ModifiedNodeKind::File { modification, .. } => modification.get_size(),
            },
            NodeDiff::Deleted(_) | NodeDiff::None => 0,
        }
    }
}

impl GetSize for FileModification {
    fn get_size(&self) -> u64 {
        match self {
            FileModification::PBO {
                dl_size,
                ..
            } => *dl_size,
            FileModification::Generic { new_length } => *new_length,
        }
    }
}
