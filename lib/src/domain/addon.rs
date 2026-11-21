use crate::infrastructure::fs::files::name_consts::ADDONS_DIR_NAME;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct AddonSettings {
    pub is_optional: bool,
    /// When set, the addon is not served by the repo. Clients are expected to
    /// subscribe to it on the Steam workshop, and it is loaded from there.
    #[serde(default)]
    pub workshop_id: Option<String>,
}

/// Where clients get an addon from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddonSource<'a> {
    /// Served by the repo, synced into `<pack>/addons/<name>`.
    Repo,
    /// Subscribed to on the Steam workshop.
    Workshop { id: &'a str },
}

impl AddonSettings {
    /// Whether clients get this addon from the Steam workshop rather than the repo.
    pub fn is_workshop(&self) -> bool {
        self.workshop_id.is_some()
    }

    pub fn source(&self) -> AddonSource<'_> {
        match &self.workshop_id {
            Some(id) => AddonSource::Workshop { id },
            None => AddonSource::Repo,
        }
    }
}

/// An addon of a specific pack, regardless of where clients get it from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackAddon {
    pub pack_name: String,
    pub name: String,
    pub settings: AddonSettings,
}

impl PackAddon {
    pub fn source(&self) -> AddonSource<'_> {
        self.settings.source()
    }

    /// The Steam workshop page of the addon, where users can subscribe to it.
    /// `None` for addons served by the repo.
    pub fn workshop_url(&self) -> Option<String> {
        match self.source() {
            AddonSource::Repo => None,
            AddonSource::Workshop { id } => Some(format!(
                "https://steamcommunity.com/sharedfiles/filedetails/?id={id}"
            )),
        }
    }

    /// The addon's folder in the repo, relative to the repo root:
    /// `<pack>/addons/<name>`. The server always has every addon there, workshop
    /// addons included.
    pub fn repo_rel_path(&self) -> PathBuf {
        PathBuf::from(&self.pack_name)
            .join(ADDONS_DIR_NAME)
            .join(&self.name)
    }
}

impl Display for PackAddon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' of pack '{}'", self.name, self.pack_name)?;

        if let AddonSource::Workshop { id } = self.source() {
            write!(f, " (workshop item {id})")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addon(workshop_id: Option<&str>) -> PackAddon {
        PackAddon {
            pack_name: "core".to_string(),
            name: "@cba".to_string(),
            settings: AddonSettings {
                workshop_id: workshop_id.map(str::to_string),
                ..Default::default()
            },
        }
    }

    #[test]
    fn workshop_url_points_at_the_workshop_page() {
        assert_eq!(
            addon(Some("450814997")).workshop_url().as_deref(),
            Some("https://steamcommunity.com/sharedfiles/filedetails/?id=450814997")
        );
        assert_eq!(addon(None).workshop_url(), None);
    }

    #[test]
    fn display_names_the_pack_and_workshop_item() {
        assert_eq!(addon(None).to_string(), "'@cba' of pack 'core'");
        assert_eq!(
            addon(Some("450814997")).to_string(),
            "'@cba' of pack 'core' (workshop item 450814997)"
        );
    }
}
