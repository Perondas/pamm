use humansize::{DECIMAL, format_size};
use pamm_lib::manifest::diff::entry_diff::{
    EntryDiff, EntryModification, FileModification, ModifiedEntryKind,
};
use pamm_lib::manifest::diff::pack_diff::PackDiff;
use pamm_lib::manifest::entries::manifest_entry::{EntryKind, ManifestEntry};
use pamm_lib::manifest::get_size::GetSize;

pub trait ToPrettyString {
    fn to_pretty_string(&self) -> String;
}

impl ToPrettyString for PackDiff {
    fn to_pretty_string(&self) -> String {
        let mut result = String::new();

        if !self.changes.is_empty() {
            result.push_str("Changes:\n");
            result.push_str(&diffs_to_string(&self.changes, ""));
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

fn diffs_to_string(diffs: &[EntryDiff], base_path: &str) -> String {
    let mut result = String::new();
    for modification in diffs {
        let added = match modification {
            EntryDiff::Created(entry) => entry_creation_to_string(entry, base_path),
            EntryDiff::Deleted(path) => {
                format!("Deleted: {}/{}\n", base_path, path)
            }
            EntryDiff::Modified(modification) => {
                entry_modification_to_string(modification, base_path)
            }
        };
        result.push_str(&added);
    }

    result
}

fn entry_creation_to_string(entry: &ManifestEntry, base_path: &str) -> String {
    let path = format!("{}/{}", base_path, entry.name);

    match &entry.kind {
        EntryKind::Folder(children) => {
            let mut result = format!("Created folder: {}\n", path);
            for part in children {
                result.push_str(&entry_creation_to_string(part, &path));
            }
            result
        }
        EntryKind::File { length, .. } => {
            format!("Created file: {} of length {}\n", path, length)
        }
    }
}

fn entry_modification_to_string(entry: &EntryModification, base_path: &str) -> String {
    let path = format!("{}/{}", base_path, entry.name);

    match &entry.kind {
        ModifiedEntryKind::Folder(children) => diffs_to_string(children, base_path),
        ModifiedEntryKind::File {
            new_length,
            modification,
            ..
        } => match modification {
            FileModification::Generic => {
                format!("Modified file: {} to new length: {}\n", path, new_length)
            }
            FileModification::PBO {
                required_parts_size,
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
