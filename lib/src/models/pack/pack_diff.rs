use crate::models::index::diff_index::diff_folder_contents;
use crate::models::index::get_dl_size::GetDlSize;
use crate::models::index::node_diff::NodeDiff;
use crate::models::pack::pack_index::PackIndex;
use anyhow::{Result, ensure};

#[derive(Debug)]
pub struct PackDiff {
    pub addon_diffs: Vec<NodeDiff>,
    pub(crate) target_index: PackIndex,
}

impl PackDiff {
    pub fn has_changes(&self) -> bool {
        self.addon_diffs
            .iter()
            .any(|c| !matches!(c, NodeDiff::None(_)))
    }
    pub fn change_count(&self) -> usize {
        self.addon_diffs
            .iter()
            .filter(|c| !matches!(c, NodeDiff::None(_)))
            .count()
    }

    pub(crate) fn get_pack_name(&self) -> &str {
        &self.target_index.pack_name
    }
}

pub fn diff_packs(old_pack: PackIndex, new_pack: PackIndex) -> Result<PackDiff> {
    ensure!(
        old_pack.pack_name == new_pack.pack_name,
        "Pack names do not match: '{}' vs '{}'",
        old_pack.pack_name,
        new_pack.pack_name
    );

    log::info!("Diffing pack '{}': local vs remote", new_pack.pack_name);

    let combined = diff_folder_contents(&old_pack.addons, &new_pack.addons)?;

    Ok(PackDiff {
        addon_diffs: combined,
        target_index: new_pack,
    })
}

impl GetDlSize for PackDiff {
    fn get_dl_size(&self) -> u64 {
        self.addon_diffs.iter().map(|diff| diff.get_dl_size()).sum()
    }
}
