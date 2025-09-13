use serde::{Deserialize, Serialize};
use crate::pack::pack_part::pack_part::PackPart;
use crate::pack::server_info::ServerInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackManifest {
    pub config: PackConfig,
    required_parts: Vec<PackPart>,
    optional_parts: Vec<PackPart>,
    servers: Vec<ServerInfo>,
    checksum: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackConfig {
    pub name: String,
    pub icon_image_path: String,
    pub icon_image_checksum: Vec<u8>,
    pub banner_image_path: String,
    pub banner_image_checksum: Vec<u8>,
    pub description: String,
    pub client_params: String,
}