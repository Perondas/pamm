use crate::commands::input::from_cli_input::FromCliInput;
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::pack::pack_config::PackConfig;
use pamm_lib::pack::server_info::ServerInfo;
use std::path::PathBuf;

impl FromCliInput for PackConfig {
    fn from_cli_input() -> Result<Self> {
        let name = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Pack Name")
            .allow_empty(true)
            .validate_with(|input: &String| -> Result<(), &str> {
                if PathBuf::from(input).exists() {
                    Err("A folder or file with this name already exists")
                } else {
                    Ok(())
                }
            })
            .interact_text()?;

        let description = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Description")
            .allow_empty(true)
            .interact_text()?;

        let client_params = vec!["-noSplash".to_string(), "-skipIntro".to_string()];
        let servers = vec![ServerInfo::default()];

        Ok(PackConfig {
            name,
            description,
            client_params,
            servers,
            remote: None,
        })
    }
}
