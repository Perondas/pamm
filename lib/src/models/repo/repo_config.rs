use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo_config_new() {
        let mut packs = HashSet::new();
        packs.insert("main_pack".to_string());

        let config = RepoConfig::new("RepoName".to_string(), "Description".to_string(), packs);
        assert_eq!(config.name, "RepoName");
        assert_eq!(config.description, "Description");
        assert!(config.packs.contains("main_pack"));
    }

    #[test]
    fn test_repo_config_to_pretty_string() {
        let mut packs = HashSet::new();
        packs.insert("main_pack".to_string());

        let config = RepoConfig::new("RepoName".to_string(), "Description".to_string(), packs);
        let pretty = config.to_pretty_string();

        assert!(pretty.contains("Repository: RepoName"));
        assert!(pretty.contains("Description: Description"));
        assert!(pretty.contains("- main_pack"));
    }
}
