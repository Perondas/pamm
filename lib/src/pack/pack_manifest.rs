use crate::fs::part_reader::read_to_part;
use crate::pack::pack_part::part::PackPart;
use crate::pack::server_info::ServerInfo;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha1::Digest;
use std::fs;
use std::path::PathBuf;

pub struct PackManifest {
    pub config: PackConfig,
    required_addons: Vec<PackPart>,
    optional_addons: Vec<PackPart>,
    icon_image: Option<PackPart>,
    banner_image: Option<PackPart>,
    pub pack_checksum: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FsPackManifest {
    pub config: PackConfig,
    required_addons: Vec<String>,
    optional_addons: Vec<String>,
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
        println!("Pack config: {:?}", config);
        let required_addons = index_addon_folder(PathBuf::from(&config.required_addons_path))?;
        let optional_addons = if let Some(optional_path) = &config.optional_addons_path {
            index_addon_folder(PathBuf::from(optional_path))?
        } else {
            vec![]
        };

        let icon_image = if let Some(icon_path) = &config.icon_image_path {
            let icon_part = read_to_part(PathBuf::from(icon_path), "" )?;
            Some(icon_part)
        } else {
            None
        };
        let banner_image = if let Some(banner_path) = &config.banner_image_path {
            let banner_part = read_to_part(PathBuf::from(banner_path), "")?;
            Some(banner_part)
        } else {
            None
        };

        let mut hasher = sha1::Sha1::new();
        for part in &required_addons {
            sha1::Digest::update(&mut hasher, part.get_checksum());
        }
        for part in &optional_addons {
            sha1::Digest::update(&mut hasher, part.get_checksum());
        }
        if let Some(icon) = &icon_image {
            sha1::Digest::update(&mut hasher, icon.get_checksum());
        }
        if let Some(banner) = &banner_image {
            sha1::Digest::update(&mut hasher, banner.get_checksum());
        }

        let pack_checksum = hasher.finalize().to_vec();

        Ok(Self {
            config,
            required_addons,
            optional_addons,
            icon_image,
            banner_image,
            pack_checksum,
        })
    }

    pub fn into_fs_manifest(self) -> (FsPackManifest, Vec<PackPart>) {
        let PackManifest {
            config,
            required_addons,
            optional_addons,
            icon_image,
            banner_image,
            pack_checksum,
        } = self;
        let required_paths = required_addons
            .iter()
            .map(|part| part.get_rel_path().to_owned())
            .collect::<Vec<_>>();
        let optional_paths = optional_addons
            .iter()
            .map(|part| part.get_rel_path().to_owned())
            .collect::<Vec<_>>();

        let fs_manifest = FsPackManifest {
            config,
            required_addons: required_paths,
            optional_addons: optional_paths,
            icon_image,
            banner_image,
            pack_checksum,
        };

        let mut all_parts = required_addons;
        all_parts.extend(optional_addons);

        (fs_manifest, all_parts)
    }
}

fn index_addon_folder(fs_path: PathBuf) -> Result<Vec<PackPart>> {
    let addon_folders = fs::read_dir(&fs_path)?;

    let mut paths = vec![];
    for entry in addon_folders {
        let entry = entry?;
        let path = entry.path();
        let url_path = path.parent().unwrap().to_str().unwrap().to_owned();


        let part = read_to_part(path, &url_path)?;
        paths.push(part);
    }
    Ok(paths)
}
