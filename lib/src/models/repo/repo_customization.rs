use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RepoCustomization {
    pub color: Option<(u32, u32, u32, u32)>,
}
