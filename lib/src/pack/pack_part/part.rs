use crate::pack::pack_part::{
    folder::Folder,
    generic_file::GenericFile,
    pbo_file::PBOFile,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PackPart {
    Folder(Folder),
    File(File),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum File {
    PBO(PBOFile),
    // TODO: maybe split large files into chunks?a
    Generic(GenericFile),
}
