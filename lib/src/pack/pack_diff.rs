use crate::pack::pack_part::part::PackPart;

#[derive(Debug)]
pub struct PackDiff {
    pub added: Vec<(String,PackPart)>,
    pub removed: Vec<String>,
    pub changed: Vec<(String,PackPart)>,
}