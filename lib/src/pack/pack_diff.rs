use crate::index::diff_index::diff_index;
use crate::index::get_size::GetSize;
use crate::index::node_diff::NodeDiff;
use crate::pack::pack_index::PackIndex;
use crate::util::iterator_diff::{DiffResult, diff_iterators};
use anyhow::Result;

pub struct PackDiff(pub Vec<NodeDiff>);

impl PackDiff {
    pub fn has_changes(&self) -> bool {
        self.0.iter().any(|c| !matches!(c, NodeDiff::None))
    }
}

pub fn diff_packs(old_pack: PackIndex, new_pack: PackIndex) -> Result<PackDiff> {
    let DiffResult {
        added,
        removed,
        same,
    } = diff_iterators(old_pack.addons, new_pack.addons, |node| node.name.clone());

    let added = added.into_iter().map(NodeDiff::Created).collect::<Vec<_>>();

    let removed = removed
        .into_iter()
        .map(|node| NodeDiff::Deleted(node.name))
        .collect::<Vec<_>>();

    let modified = same
        .into_iter()
        .map(|(old, new)| diff_index(old, new))
        .collect::<Result<Vec<_>>>()?;

    let combined = added.into_iter().chain(removed).chain(modified).collect();
    Ok(PackDiff(combined))
}

impl GetSize for PackDiff {
    fn get_size(&self) -> u64 {
        self.0.iter().map(|diff| diff.get_size()).sum()
    }
}
