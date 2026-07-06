use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ServerConfig {
    pub server_dir: Option<PathBuf>,
    /// Path where the string is to be stored mapped to the template to be filled out
    pub script_templates: HashMap<PathBuf, String>,
    /// Will be run in order after deployment of any pack
    pub post_deploy_commands: Vec<String>,
}
