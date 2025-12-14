use crate::manifest::diff::entry_diff::{EntryDiff, ModifiedEntryKind};
use crate::manifest::diff::pack_diff::PackDiff;
use crate::manifest::entries::manifest_entry::{EntryKind, ManifestEntry};

pub trait GetSize {
    fn get_size(&self) -> u64;
}

impl GetSize for ManifestEntry {
    fn get_size(&self) -> u64 {
        match &self.kind {
            EntryKind::File { length, .. } => *length,
            EntryKind::Folder(children) => children.iter().map(|child| child.get_size()).sum(),
        }
    }
}

impl GetSize for EntryDiff {
    fn get_size(&self) -> u64 {
        match self {
            EntryDiff::Created(entry) => entry.get_size(),
            EntryDiff::Deleted(_) => 0,
            EntryDiff::Modified(modification) => match &modification.kind {
                ModifiedEntryKind::Folder(e) => e.iter().map(|child| child.get_size()).sum(),
                ModifiedEntryKind::File { new_length, .. } => *new_length,
            },
        }
    }
}

impl GetSize for PackDiff {
    fn get_size(&self) -> u64 {
        self.changes.iter().map(|change| change.get_size()).sum()
    }
}
