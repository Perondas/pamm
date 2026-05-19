use indicatif::DecimalBytes;
use pamm_lib::models::index::get_dl_size::GetDlSize;
use pamm_lib::models::index::get_size_change::GetSizeChange;
use pamm_lib::models::index::index_node::{IndexNode, NodeKind};
use pamm_lib::models::index::node_diff::{
    FileModification, ModifiedNodeKind, NodeDiff, NodeModification,
};
use pamm_lib::models::pack::pack_diff::PackDiff;

pub trait ToPrettyString {
    fn to_pretty_string(&self) -> String;
}

impl ToPrettyString for PackDiff {
    fn to_pretty_string(&self) -> String {
        let mut result = String::new();

        let PackDiff {
            addon_diffs: changes,
            ..
        } = self;

        if !changes.is_empty() {
            result.push_str("Changes:\n");
            result.push_str(&diffs_to_string(changes, ""));
            result.push('\n');
            result.push_str(&totals_to_string(self.get_dl_size(), self.get_size_change()));
        }

        result
    }
}

impl ToPrettyString for Vec<PackDiff> {
    fn to_pretty_string(&self) -> String {
        let mut result = String::new();

        let name_width = self
            .iter()
            .map(|d| d.get_pack_name().len())
            .max()
            .unwrap_or(0);

        result.push_str("Pack summary:\n");
        for diff in self {
            let name = diff.get_pack_name();
            if diff.has_changes() {
                let size_change = diff.get_size_change();
                result.push_str(&format!(
                    "  {:<width$}  {} change(s), {} to download, {}{} size change\n",
                    name,
                    diff.change_count(),
                    DecimalBytes(diff.get_dl_size()),
                    if size_change.is_negative() { "-" } else { "+" },
                    DecimalBytes(size_change.unsigned_abs()),
                    width = name_width,
                ));
            } else {
                result.push_str(&format!(
                    "  {:<width$}  no changes\n",
                    name,
                    width = name_width,
                ));
            }
        }

        let total_dl: u64 = self.iter().map(|d| d.get_dl_size()).sum();
        let total_change: i64 = self.iter().map(|d| d.get_size_change()).sum();

        result.push('\n');
        result.push_str(&totals_to_string(total_dl, total_change));

        result
    }
}

pub fn multi_pack_details_string(diffs: &[PackDiff]) -> String {
    let mut result = String::new();

    for diff in diffs {
        result.push_str(&format!("=== Pack: {} ===\n", diff.get_pack_name()));
        if diff.has_changes() {
            result.push_str(&diffs_to_string(&diff.addon_diffs, ""));
        } else {
            result.push_str("No changes.\n");
        }
        result.push('\n');
    }

    result
}

fn totals_to_string(dl_size: u64, size_change: i64) -> String {
    format!(
        "Total download size: {}\nTotal size change: {}{}",
        DecimalBytes(dl_size),
        if size_change.is_negative() { "-" } else { "+" },
        DecimalBytes(size_change.unsigned_abs()),
    )
}

fn diffs_to_string(diffs: &[NodeDiff], base_path: &str) -> String {
    let mut result = String::new();
    for modification in diffs {
        let added = match modification {
            NodeDiff::Created(entry) => entry_creation_to_string(entry, base_path),
            NodeDiff::Deleted { name: path, .. } => {
                format!("Deleted: {}/{}\n", base_path, path)
            }
            NodeDiff::Modified(modification) => {
                entry_modification_to_string(modification, base_path)
            }
            NodeDiff::None(_) => "".to_string(),
        };
        result.push_str(&added);
    }

    result
}

fn entry_creation_to_string(entry: &IndexNode, base_path: &str) -> String {
    let path = format!("{}/{}", base_path, entry.name);

    match &entry.kind {
        NodeKind::Folder(children) => {
            let mut result = format!("Created folder: {}\n", path);
            for part in children {
                result.push_str(&entry_creation_to_string(part, &path));
            }
            result
        }
        NodeKind::File { length, .. } => {
            format!(
                "Created file: {} of length {}\n",
                path,
                DecimalBytes(*length)
            )
        }
    }
}

fn entry_modification_to_string(entry: &NodeModification, base_path: &str) -> String {
    let path = format!("{}/{}", base_path, entry.name);

    match &entry.kind {
        ModifiedNodeKind::Folder(children) => diffs_to_string(children, base_path),
        ModifiedNodeKind::File { modification, .. } => match modification {
            FileModification::Generic { new_length } => {
                format!(
                    "Modified file: {} to new length: {}\n",
                    path,
                    DecimalBytes(*new_length)
                )
            }
            FileModification::PBO {
                dl_size,
                new_length,
                ..
            } => {
                format!(
                    "Modified PBO file: {} with to new length: {}\nThis PBO patch requires {} of data.\n",
                    path,
                    DecimalBytes(*new_length),
                    DecimalBytes(*dl_size)
                )
            }
        },
    }
}
