use crate::io::fs::util::try_find_addon_name::try_find_addon_name;
use anyhow::ensure;
use log::warn;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub struct ExternalAddon {
    pub path: PathBuf,
    pub name: Option<String>,
    pub enabled: bool,
    #[serde(default)]
    pub found_in_workshop: bool,
    // For convenience we want this to stay constructable in tests, but nowhere else
}

impl ExternalAddon {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        ensure!(
            path.as_ref().is_dir(),
            "External addon path is not a directory: {:#?}",
            path.as_ref()
        );

        let name = try_find_addon_name(&path).unwrap_or_else(|_| {
            warn!("Failed to read addon name for path: {:?}", path.as_ref());
            None
        });

        Ok(ExternalAddon {
            path: path.as_ref().to_path_buf(),
            name,
            enabled: false,
            found_in_workshop: false,
        })
    }

    pub fn with_enabled(self, enabled: bool) -> Self {
        Self { enabled, ..self }
    }

    pub fn found_in_workshop(self) -> Self {
        Self {
            found_in_workshop: true,
            ..self
        }
    }
}

impl PartialEq for ExternalAddon {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for ExternalAddon {}

impl Hash for ExternalAddon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.path.as_os_str().as_encoded_bytes());
    }
}
