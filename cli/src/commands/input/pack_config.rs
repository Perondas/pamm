use crate::commands::input::from_cli_input::FromCliInput;
use anyhow::Result;
use pamm_lib::pack::pack_manifest::PackConfig;
use pamm_lib::pack::server_info::ServerInfo;
use std::path::PathBuf;

impl FromCliInput for PackConfig {
    fn from_cli_input() -> Result<Self> {
        let name = dialoguer::Input::<String>::new()
            .with_prompt("Name")
            .interact_text()?;

        let description = dialoguer::Input::<String>::new()
            .with_prompt("Description")
            .allow_empty(true)
            .interact_text()?;

        let icon_image_path = dialoguer::Input::<String>::new()
            .with_prompt("Path to the icon image")
            .allow_empty(true)
            .interact_text()?;

        let banner_image_path = dialoguer::Input::<String>::new()
            .with_prompt("Path to the banner image")
            .allow_empty(true)
            .interact_text()?;

        let client_params = dialoguer::Input::<String>::new()
            .with_prompt("Client parameters (e.g. -noSplash -skipIntro)")
            .allow_empty(true)
            .interact_text()?;

        let required_mods_path = dialoguer::Input::<String>::new()
            .with_prompt("Path to the required mods folder")
            .allow_empty(false)
            .validate_with(|input: &String| -> Result<(), &str> {
                if PathBuf::from(input).is_dir() {
                    Ok(())
                } else {
                    Err("Path must be a directory")
                }
            })
            .interact_text()?;

        let optional_mods_path = dialoguer::Input::<String>::new()
            .with_prompt("Path to the optional mods folder")
            .allow_empty(true)
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.is_empty() || PathBuf::from(input).is_dir() {
                    Ok(())
                } else {
                    Err("Path must be a directory")
                }
            })
            .interact_text()?;

        let mut servers = vec![];

        loop {
            let add_server = dialoguer::Select::new()
                .with_prompt("Do you want to add a server to the pack?")
                .items(["Yes", "No"])
                .default(1)
                .interact()?;

            if add_server == 1 {
                break;
            }

            servers.push(ServerInfo::from_cli_input()?);
        }

        Ok(PackConfig {
            name,
            icon_image_path: if icon_image_path.is_empty() {
                None
            } else {
                Some(PathBuf::from(icon_image_path))
            },
            banner_image_path: if banner_image_path.is_empty() {
                None
            } else {
                Some(PathBuf::from(banner_image_path))
            },
            required_parts_path: PathBuf::from(required_mods_path),
            optional_parts_path: if optional_mods_path.is_empty() {
                None
            } else {
                Some(PathBuf::from(optional_mods_path))
            },
            description,
            client_params,
            servers,
        })
    }
}
