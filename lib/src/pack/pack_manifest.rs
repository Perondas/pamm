use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::pack::pack_part::part::PackPart;
use crate::pack::server_info::ServerInfo;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackManifest {
    pub config: PackConfig,
    required_addons: Vec<PathBuf>,
    optional_addons: Vec<PathBuf>,
    icon_image: Option<PackPart>,
    banner_image: Option<PackPart>,
    pub pack_checksum: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackConfig {
    pub name: String,
    pub required_addons_path: PathBuf,
    pub optional_addons_path: Option<PathBuf>,
    pub icon_image_path: Option<PathBuf>,
    pub banner_image_path: Option<PathBuf>,
    pub description: String,
    pub client_params: String,
    pub servers: Vec<ServerInfo>,
}


impl PackManifest {
    pub fn new(config: PackConfig) -> Result<Self> {
        
    }
}