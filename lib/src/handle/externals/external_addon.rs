use crate::io::fs::util::try_find_addon_name::try_find_addon_name;
use anyhow::{bail, ensure};
use log::warn;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
#[cfg(not(test))]
use std::marker::PhantomData;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub struct ExternalAddon {
    pub path: String,
    pub name: Option<String>,
    pub enabled: bool,
    #[cfg(not(test))]
    pub phantom: PhantomData<()>,
}

impl ExternalAddon {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        ensure!(
            path.as_ref().is_dir(),
            "External addon path is not a directory: {:?}",
            path.as_ref()
        );

        let name = try_find_addon_name(&path).unwrap_or_else(|_| {
            warn!("Failed to read addon name for path: {:?}", path.as_ref());
            None
        });

        Ok(ExternalAddon {
            path: path.as_ref().to_string_lossy().to_string(),
            name,
            enabled: true,
            #[cfg(not(test))]
            phantom: PhantomData,
        })
    }

    pub fn with_enabled(self, enabled: bool) -> Self {
        Self { enabled, ..self }
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
        state.write(self.path.as_bytes());
    }
}
