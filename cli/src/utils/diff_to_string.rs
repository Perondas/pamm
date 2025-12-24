use humansize::{DECIMAL, format_size};
use pamm_lib::index::get_size::GetSize;
use pamm_lib::index::index_node::{IndexNode, NodeKind};
use pamm_lib::index::node_diff::{FileModification, ModifiedNodeKind, NodeDiff, NodeModification};
use pamm_lib::pack::pack_diff::PackDiff;

pub trait ToPrettyString {
    fn to_pretty_string(&self) -> String;
}

impl ToPrettyString for PackDiff {
    fn to_pretty_string(&self) -> String {
        let mut result = String::new();

        let PackDiff(changes) = self;

        if !changes.is_empty() {
            result.push_str("Changes:\n");
            result.push_str(&diffs_to_string(changes, ""));
            result.push('\n');
            result.push_str(
                format!(
                    "Total change size: {}",
                    format_size(self.get_size(), DECIMAL)
                )
                .as_str(),
            );
        }

        result
    }
}

fn diffs_to_string(diffs: &[NodeDiff], base_path: &str) -> String {
    let mut result = String::new();
    for modification in diffs {
        let added = match modification {
            NodeDiff::Created(entry) => entry_creation_to_string(entry, base_path),
            NodeDiff::Deleted(path) => {
                format!("Deleted: {}/{}\n", base_path, path)
            }
            NodeDiff::Modified(modification) => {
                entry_modification_to_string(modification, base_path)
            }
            NodeDiff::None => "".to_string(),
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
            format!("Created file: {} of length {}\n", path, length)
        }
    }
}

fn entry_modification_to_string(entry: &NodeModification, base_path: &str) -> String {
    let path = format!("{}/{}", base_path, entry.name);

    match &entry.kind {
        ModifiedNodeKind::Folder(children) => diffs_to_string(children, base_path),
        ModifiedNodeKind::File { modification, .. } => match modification {
            FileModification::Generic { new_length } => {
                format!("Modified file: {} to new length: {}\n", path, new_length)
            }
            FileModification::PBO {
                required_parts_size,
                new_length,
                ..
            } => {
                format!(
                    "Modified PBO file: {} with to new length: {}\nThis PBO patch requires {} bytes of data.\n",
                    path, new_length, required_parts_size
                )
            }
        },
    }
}
