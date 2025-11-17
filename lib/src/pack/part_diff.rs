use crate::pack::pack_part::part::PackPart;
use crate::pack::pack_part::pbo_file::PBOPart;

#[derive(Debug)]
pub enum PartDiff {
    Created(PackPart),
    Deleted(String),
    Modified(PartModification),
}

#[derive(Debug)]
pub enum PartModification {
    Folder(FolderModification),
    File(FileModification),
}

#[derive(Debug)]
pub struct FolderModification {
    pub name: String,
    pub changes: Vec<PartDiff>,
}

#[derive(Debug)]
pub enum FileModification {
    PBO(PBOModification),
    GenericFile(GenericFileModification),
}

#[derive(Debug)]
pub struct PBOModification {
    pub name: String,
    // Order of the parts by checksum
    pub new_order: Vec<PBOPart>,
    pub required_parts: Vec<Vec<u8>>,
    pub target_checksum: Vec<u8>,
    pub blob_offset: u64,
}

#[derive(Debug)]
pub struct GenericFileModification {
    pub name: String,
    pub new_length: u64,
    pub target_checksum: Vec<u8>,
}
