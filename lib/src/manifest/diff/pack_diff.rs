use crate::manifest::diff::entry_diff::{
    EntryDiff, EntryModification, FileModification, ModifiedEntryKind,
};
use crate::manifest::entries::manifest_entry::{EntryKind, ManifestEntry};

#[derive(Debug)]
pub struct PackDiff {
    pub changes: Vec<EntryDiff>,
}

impl PackDiff {
    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }

    pub fn to_pretty_string(&self) -> String {
        let mut result = String::new();

        if !self.changes.is_empty() {
            result.push_str(" Changes:\n");
            result.push_str(&diffs_to_string(&self.changes, ""));
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
