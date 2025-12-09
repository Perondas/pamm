use crate::commands::input::from_cli_input::FromCliInput;
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::repo::repo_config::RepoConfig;
use std::path::PathBuf;

impl FromCliInput for RepoConfig {
    fn from_cli_input() -> Result<Self> {
        let name = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Repo Name")
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

        Ok(RepoConfig::new(name, description, vec![]))
    }
}
