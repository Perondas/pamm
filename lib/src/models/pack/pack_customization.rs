use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PackCustomization {
    /// File name of the pack's icon inside the repo's `media/` directory.
    pub icon: Option<String>,
    /// File name of the pack's banner inside the repo's `media/` directory.
    pub banner: Option<String>,
}

impl PackCustomization {
    /// Names of all media files this customization references.
    pub fn media_file_names(&self) -> Vec<&str> {
        [self.icon.as_deref(), self.banner.as_deref()]
            .into_iter()
            .flatten()
            .collect()
    }
}
