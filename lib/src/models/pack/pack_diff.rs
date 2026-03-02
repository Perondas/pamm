use crate::models::index::diff_index::diff_index;
use crate::models::index::get_size::GetSize;
use crate::models::index::node_diff::NodeDiff;
use crate::models::pack::pack_index::PackIndex;
use crate::util::iterator_diff::{diff_iterators, DiffResult};
use anyhow::{ensure, Result};

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

    let DiffResult {
        added,
        removed,
        same,
    } = diff_iterators(&old_pack.addons, &new_pack.addons, |node| node.name.clone());

    log::debug!(
        "Pack diff: {} added, {} removed, {} to check for modifications",
        added.len(),
        removed.len(),
        same.len()
    );

    let added = added
        .into_iter()
        .cloned()
        .map(NodeDiff::Created)
        .collect::<Vec<_>>();

    let removed = removed
        .into_iter()
        .map(|node| NodeDiff::Deleted(node.name.to_string()))
        .collect::<Vec<_>>();

    let modified = same
        .into_iter()
        .map(|(old, new)| diff_index(old, new))
        .collect::<Result<Vec<_>>>()?;

    let combined = added.into_iter().chain(removed).chain(modified).collect();

    Ok(PackDiff {
        addon_diffs: combined,
        target_index: new_pack,
    })
}

impl GetSize for PackDiff {
    fn get_size(&self) -> u64 {
        self.addon_diffs.iter().map(|diff| diff.get_size()).sum()
    }
}
