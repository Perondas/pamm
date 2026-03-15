use crate::models::index::index_node::{IndexNode, NodeKind};
use crate::models::index::node_diff::{FileModification, ModifiedNodeKind, NodeDiff};

pub trait GetDlSize {
    fn get_dl_size(&self) -> u64;
}

impl GetDlSize for IndexNode {
    fn get_dl_size(&self) -> u64 {
        match &self.kind {
            NodeKind::File { length, .. } => *length,
            NodeKind::Folder(children) => children.iter().map(|child| child.get_dl_size()).sum(),
        }
    }
}

impl GetDlSize for NodeDiff {
    fn get_dl_size(&self) -> u64 {
        match self {
            NodeDiff::Created(entry) => entry.get_dl_size(),
            NodeDiff::Modified(modification) => match &modification.kind {
                ModifiedNodeKind::Folder(e) => e.iter().map(|child| child.get_dl_size()).sum(),
                ModifiedNodeKind::File { modification, .. } => modification.get_dl_size(),
            },
            NodeDiff::Deleted(_) | NodeDiff::None(_) => 0,
        }
    }
}

impl GetDlSize for FileModification {
    fn get_dl_size(&self) -> u64 {
        match self {
            FileModification::PBO {
                dl_size,
                ..
            } => *dl_size,
            FileModification::Generic { new_length } => *new_length,
        }
    }
}
