use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoConfig {
    pub name: String,
    pub description: String,
    /// Store the names of packs available in this repo
    pub packs: Vec<String>,
}

impl RepoConfig {
    pub fn new(name: String, description: String, packs: Vec<String>) -> Self {
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
