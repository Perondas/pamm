use crate::pack::pack_part::folder::Folder;
use crate::pack::pack_part::part::{File, PackPart};
use crate::pack::part_diff::{FileModification, PartDiff, PartModification};

#[derive(Debug)]
pub struct PackDiff {
    pub required_changes: Vec<PartDiff>,
    pub optional_changes: Vec<PartDiff>,
}

impl PackDiff {
    pub fn has_changes(&self) -> bool {
        !self.required_changes.is_empty() || !self.optional_changes.is_empty()
    }

    pub fn to_pretty_string(&self) -> String {
        let mut result = String::new();

        if !self.required_changes.is_empty() {
            result.push_str("Required Changes:\n");
            result.push_str(&diff_to_string(&self.required_changes, ""));
        }
        if !self.optional_changes.is_empty() {
            result.push_str("Optional Changes:\n");
            result.push_str(&diff_to_string(&self.optional_changes, ""));
        }

        result
    }
}

fn diff_to_string(diffs: &[PartDiff], base_path: &str) -> String {
    let mut result = String::new();
    for modification in diffs {
        let added = match modification {
            PartDiff::Created(part) => match part {
                PackPart::Folder(f) => folder_creation_to_string(f, base_path),
                PackPart::File(f) => file_creation_to_string(f, base_path),
            },
            PartDiff::Deleted(path) => {
                format!("Deleted: {}/{}\n", base_path, path)
            }
            PartDiff::Modified(modification) => match modification {
                PartModification::Folder(f) => {
                    let folder_path = format!("{}/{}", base_path, f.name);
                    diff_to_string(&f.changes, &folder_path)
                }
                PartModification::File(f) => match f {
                    FileModification::PBO(p) => pbo_patch_to_string(p, base_path),
                    FileModification::GenericFile(g) => {
                        let path = format!("{}/{}", base_path, g.name);
                        format!("Modified file: {} to new length: {}\n", path, g.new_length)
                    }
                },
            },
        };
        result.push_str(&added);
    }
    result
}

fn folder_creation_to_string(folder: &Folder, base_path: &str) -> String {
    let folder_path = format!("{}/{}", base_path, folder.name);
    let mut result = format!("Created folder: {}\n", folder_path);
    for part in &folder.children {
        match part {
            PackPart::Folder(f) => {
                result.push_str(&folder_creation_to_string(f, &folder_path));
            }
            PackPart::File(f) => {
                result.push_str(&file_creation_to_string(f, &folder_path));
            }
        }
    }
    result
}

fn file_creation_to_string(file: &File, base_path: &str) -> String {
    format!(
        "Crate file {}/{} of length {}\n",
        base_path,
        file.get_name(),
        file.get_length(),
    )
}

fn pbo_patch_to_string(
    pbo_modification: &crate::pack::part_diff::PBOModification,
    base_path: &str,
) -> String {
    let file_path = format!("{}/{}", base_path, pbo_modification.name);
    format!(
        "Modified PBO file: {} with to new length: {}\nThis PBO patch requires {} bytes of data.\n",
        file_path, pbo_modification.new_length, pbo_modification.required_parts_size
    )
}
