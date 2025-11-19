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
    // TODO: maybe split large files into chunks?
    Generic(GenericFile),
}

impl PackPart {
    pub fn get_checksum(&self) -> Vec<u8> {
        match self {
            PackPart::Folder(folder) => folder.checksum.clone(),
            PackPart::File(file) => file.get_checksum().clone(),
        }
    }
    
    pub fn get_name(&self) -> &str {
        match self {
            PackPart::Folder(folder) => &folder.name,
            PackPart::File(file) =>file.get_name(),
        }
    }
}

impl File {
    pub fn get_name(&self) -> &str {
        match self {
            File::PBO(pbo) => &pbo.name,
            File::Generic(generic) => &generic.name,
        }
    }

    pub fn get_checksum(&self) -> Vec<u8> {
        match self {
            File::PBO(pbo) => pbo.checksum.clone(),
            File::Generic(generic) => generic.checksum.clone(),
        }
    }

    pub fn get_length(&self) -> u64 {
        match self {
            File::PBO(pbo) => pbo.length,
            File::Generic(generic) => generic.length,
        }
    }
}
