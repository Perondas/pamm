use crate::migration::migrations::v0tov1::v0_to_v1;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum VersionTag {
    #[default]
    V0,
    V1,
}

impl VersionTag {
    pub fn is_latest(&self) -> bool {
        matches!(self, VersionTag::V1)
    }

    pub fn get_latest() -> Self {
        VersionTag::V1
    }

    pub fn get_migration_function(&self) -> fn(&std::path::Path) -> anyhow::Result<Self> {
        match self {
            VersionTag::V0 => v0_to_v1,
            VersionTag::V1 => |_path| Err(anyhow!("Cannot migrate from latest version")),
        }
    }
}
