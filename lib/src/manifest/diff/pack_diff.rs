use crate::manifest::diff::entry_diff::EntryDiff;

#[derive(Debug)]
pub struct PackDiff {
    pub changes: Vec<EntryDiff>,
}

impl PackDiff {
    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }
}
