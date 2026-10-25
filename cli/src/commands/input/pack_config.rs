use crate::commands::input::from_cli_input::FromCliInputWithContext;
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::io::files::file_names::fixed_file::FixedFile;
use pamm_lib::models::pack::pack_config::PackConfig;
use std::path::PathBuf;

impl FromCliInputWithContext for PackConfig {
    fn from_cli_input(existing_packs: &[&str]) -> Result<Self> {
        let name = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Pack Name")
            .allow_empty(true)
            .validate_with(|_input: &String| -> Result<(), &str> {
                let path = PackConfig::file_name();
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

        let mut parent = None;

        if !existing_packs.is_empty() {
            // We don't check for loops because we assume that users will never edit the pack config manually
            parent = dialoguer::Select::new()
                .with_prompt("Select Parent Pack (if any) ESC to skip")
                .items(existing_packs)
                .interact_opt()?
                .map(|i| existing_packs[i].to_string());
        }

        Ok(PackConfig::new(
            name,
            description,
            client_params,
            parent,
            vec![],
        ))
    }
}
