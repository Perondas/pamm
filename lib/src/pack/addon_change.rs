use crate::pack::pack_part::part::PackPart;

pub enum AddonChange {
    Added(PackPart),
    Removed(String),
    Modified(PackPart)
}