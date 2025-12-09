use crate::commands::input::from_cli_input::{FromCliInput, FromCliInputWithContext};
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::named::Named;
use pamm_lib::pack::pack_config::PackConfig;
use pamm_lib::pack::server_info::ServerInfo;
use pamm_lib::repo::repo_config::RepoConfig;
use std::path::PathBuf;

impl FromCliInputWithContext for PackConfig {
    fn from_cli_input(repo_config: &RepoConfig) -> Result<Self> {
        let name = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Repo Name")
            .allow_empty(true)
            .validate_with(|input: &String| -> Result<(), &str> {
                let path = PackConfig::get_name(input);
                if PathBuf::from(path).exists() {
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

        let client_params = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Client Prams")
            .default("-noSplash -skipIntro".to_string())
            .allow_empty(true)
            .interact_text()?
            .split(" ")
            .map(|s| s.to_string())
            .collect();

        let mut servers = vec![];

        loop {
            let confirm = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to add a server?".to_string())
                .default(false)
                .interact()?;

            if !confirm {
                break;
            } else {
                servers.push(ServerInfo::from_cli_input()?);
            }
        }

        // We don't check for loops because we assume that users will never edit the pack config manually
        let parent = dialoguer::Select::new()
            .with_prompt("Select Parent Pack (if any)")
            .items(&repo_config.packs)
            .interact_opt()?
            .map(|i| repo_config.packs[i].clone());

        Ok(PackConfig::new(
            name,
            description,
            client_params,
            servers,
            parent,
        ))
    }
}
