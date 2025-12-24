use crate::index::index_node::{IndexNode, NodeKind};
use crate::index::node_diff::{ModifiedNodeKind, NodeDiff, NodeModification};
use crate::io::net::remote_patcher::RemotePatcher;
use crate::io::rel_path::RelPath;
use crate::pack::pack_diff::PackDiff;
use anyhow::Context;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::fs;
use std::path::PathBuf;

pub struct DiffApplier {
    base_dir: PathBuf,
    remote_patcher: RemotePatcher,
}

impl DiffApplier {
    pub fn new(base_dir: PathBuf, remote_patcher: RemotePatcher) -> Self {
        Self {
            base_dir,
            remote_patcher,
        }
    }

    pub fn apply(&self, diff: PackDiff) -> anyhow::Result<()> {
        let PackDiff(node_diffs) = diff;

        let res = node_diffs
            .into_par_iter()
            .map(|diff| self.apply_node_diff(diff, RelPath::new()))
            .collect::<Vec<_>>();

        let errors: Vec<anyhow::Error> = res.into_iter().filter_map(|r| r.err()).collect();

        if !errors.is_empty() {
            let combined_error = errors.into_iter().fold(
                anyhow::anyhow!("One or more errors occurred while applying diff"),
                |acc, err| acc.context(err),
            );
            return Err(combined_error);
        }

        Ok(())
    }

    fn apply_node_diff(&self, node_diff: NodeDiff, parent_path: RelPath) -> anyhow::Result<()> {
        match node_diff {
            NodeDiff::Created(node) => self.create_node(node, parent_path),
            NodeDiff::Deleted(name) => self.delete_node(name, parent_path),
            NodeDiff::Modified(modification) => self.apply_modification(modification, parent_path),
            NodeDiff::None => Ok(()),
        }
    }

    fn delete_node(&self, name: String, parent_path: RelPath) -> anyhow::Result<()> {
        let path = parent_path.push(&name);
        let full_path = path.with_base_path(&self.base_dir);

        if full_path.is_dir() {
            fs::remove_dir_all(&full_path).context("Failed to delete directory")
        } else if full_path.is_file() {
            fs::remove_file(&full_path).context("Failed to delete file")
        } else {
            unreachable!(
                "Path to delete is neither file nor directory: {:?}",
                full_path
            );
        }
    }

    fn create_node(&self, node: IndexNode, parent_path: RelPath) -> anyhow::Result<()> {
        let path = parent_path.push(&node.name);

        match node.kind {
            NodeKind::File { length, .. } => {
                self.remote_patcher
                    .create_file(&path, &path.with_base_path(&self.base_dir), length)
            }
            NodeKind::Folder(children) => {
                for child in children {
                    self.create_node(child, path.clone())?;
                }
                Ok(())
            }
        }
    }

    fn apply_modification(
        &self,
        modification: NodeModification,
        parent_path: RelPath,
    ) -> anyhow::Result<()> {
        let path = parent_path.push(&modification.name);
        let file_path = path.with_base_path(&self.base_dir);

        match modification.kind {
            ModifiedNodeKind::Folder(children) => {
                for child in children {
                    self.apply_node_diff(child, path.clone())?;
                }
            }
            ModifiedNodeKind::File { modification, .. } => {
                self.remote_patcher
                    .patch_file(&path, &file_path, modification)?;
            }
        }

        Ok(())
    }
}
