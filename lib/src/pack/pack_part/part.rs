use crate::pack::pack_part::{
    folder::Folder,
    generic_file::GenericFile,
    pbo_file::PBOFile,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PackPart {
    Folder(Folder),
    File(File),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum File {
    PBO(PBOFile),
    // TODO: maybe split large files into chunks?a
    Generic(GenericFile),
}

impl PackPart {
    pub fn get_checksum(&self) -> Vec<u8> {
        match self {
            PackPart::Folder(folder) => folder.checksum.clone(),
            PackPart::File(file) => file.get_checksum().clone(),
        }
    }
    
    pub fn get_rel_path(&self) -> &str {
        match self {
            PackPart::Folder(folder) => &folder.rel_path,
            PackPart::File(file) =>file.get_rel_path(),
        }
    }
}

impl File {
    pub fn get_rel_path(&self) -> &str {
        match self {
            File::PBO(pbo) => &pbo.rel_path,
            File::Generic(generic) => &generic.rel_path,
        }
    }

    pub fn get_checksum(&self) -> Vec<u8> {
        match self {
            File::PBO(pbo) => pbo.checksum.clone(),
            File::Generic(generic) => generic.checksum.clone(),
        }
    }
}
