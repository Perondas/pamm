use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::pack::pack_part::part::PackPart;
use crate::pack::server_info::ServerInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackManifest {
    pub config: PackConfig,
    required_parts: Vec<PackPart>,
    optional_parts: Vec<PackPart>,
    icon_image_data: Option<PackPart>,
    banner_image_data: Option<PackPart>,
    pub pack_checksum: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackConfig {
    pub name: String,
    pub required_parts_path: PathBuf,
    pub optional_parts_path: Option<PathBuf>,
    pub icon_image_path: Option<PathBuf>,
    pub banner_image_path: Option<PathBuf>,
    pub description: String,
    pub client_params: String,
    pub servers: Vec<ServerInfo>,
}
