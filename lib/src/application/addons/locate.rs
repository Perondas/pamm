use crate::domain::addon::{AddonSource, PackAddon};
use crate::platform::steam::get_workshop_item_path::get_workshop_item_path;
use std::path::{Path, PathBuf};

impl PackAddon {
    /// Finds the addon on this machine: in the repo for served addons, in the
    /// Steam workshop folder for workshop addons. `None` when it is not there,
    /// which means the pack is definitely out of sync. Finding the addon says
    /// nothing about whether it is up to date, so `Some` does not mean the pack
    /// is in sync.
    pub(crate) fn locate(&self, repo_path: &Path) -> anyhow::Result<Option<PathBuf>> {
        match self.source() {
            AddonSource::Repo => {
                let path = repo_path.join(self.repo_rel_path());
                Ok(path.exists().then_some(path))
            }
            AddonSource::Workshop { id } => get_workshop_item_path(id),
        }
    }
}
