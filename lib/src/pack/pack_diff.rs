use crate::pack::part_diff::PartDiff;

#[derive(Debug)]
pub struct PackDiff {
    pub required_changes: Vec<PartDiff>,
    pub optional_changes: Vec<PartDiff>,
}

impl PackDiff {
    pub fn has_changes(&self) -> bool {
        !self.required_changes.is_empty() && !self.optional_changes.is_empty()
    }
}
