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

impl PackPart {
    pub fn get_checksum(&self) -> &[u8] {
        match self {
            PackPart::Folder(folder) => &folder.checksum,
            PackPart::File(file) => match file {
                File::PBO(pbo) => &pbo.checksum,
                File::Generic(generic) => &generic.checksum,
            },
        }
    }
    
    pub fn get_rel_path(&self) -> &str {
        match self {
            PackPart::Folder(folder) => &folder.rel_path,
            PackPart::File(file) => match file {
                File::PBO(pbo) => &pbo.rel_path,
                File::Generic(generic) => &generic.rel_path,
            },
        }
    }
}
