use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RepoCustomization {
    pub color: Option<(u32, u32, u32, u32)>,
    /// File name of the repo's icon inside the repo's `media/` directory.
    pub icon: Option<String>,
    /// File name of the repo's banner inside the repo's `media/` directory.
    pub banner: Option<String>,
}

impl RepoCustomization {
    /// Names of all media files this customization references.
    pub fn media_file_names(&self) -> Vec<&str> {
        [self.icon.as_deref(), self.banner.as_deref()]
            .into_iter()
            .flatten()
            .collect()
    }
}
