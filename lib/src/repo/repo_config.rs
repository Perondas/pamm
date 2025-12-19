use crate::{hr_serializable, known_name};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

hr_serializable!(RepoConfig);
known_name!(RepoConfig, "repo.config.json");
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoConfig {
    pub name: String,
    pub description: String,
    /// Store the names of packs available in this repo
    pub packs: HashSet<String>,
}

impl RepoConfig {
    pub fn new(name: String, description: String, packs: HashSet<String>) -> Self {
        RepoConfig {
            name,
            description,
            packs,
        }
    }

    pub fn to_pretty_string(&self) -> String {
        let mut result = format!(
            "Repository: {}\nDescription: {}\nPacks:\n",
            self.name, self.description
        );
        for pack in &self.packs {
            result.push_str(&format!("- {}\n", pack));
        }
        result
    }
}
