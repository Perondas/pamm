use crate::handle::repo_handle::RepoHandle;
use crate::io::files::file_paths::rel_path::RelPath;
use crate::io::net::remote_patcher::RemotePatcher;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::models::index::get_dl_size::GetDlSize;
use crate::models::index::index_node::{IndexNode, NodeKind};
use crate::models::index::node_diff::{ModifiedNodeKind, NodeDiff, NodeModification};
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_diff::PackDiff;
use anyhow::Context;
use futures::future::BoxFuture;
use futures::{FutureExt, StreamExt, stream};
use std::cmp::Ordering;
use std::path::PathBuf;
use tokio::fs;
use url::Url;

/// How many addons are synced at once. Each one downloads its files in turn.
const CONCURRENT_ADDON_SYNCS: usize = 8;

pub struct DiffApplier<P: ProgressReporter> {
    addon_dir: PathBuf,
    remote_patcher: RemotePatcher<P>,
    progress_reporter: P,
}

impl PackConfig {
    pub fn diff_applier<P: ProgressReporter>(
        &self,
        repo_handle: &RepoHandle,
        base_url: &Url,
        progress_reporter: P,
    ) -> DiffApplier<P> {
        let addon_dir = repo_handle.pack_addons_path(&self.name);
        let remote_patcher = self.remote_patcher(base_url, progress_reporter.clone());

        DiffApplier::new(addon_dir, remote_patcher, progress_reporter)
    }
}

impl<P: ProgressReporter> DiffApplier<P> {
    pub(crate) fn new(
        addon_dir: PathBuf,
        remote_patcher: RemotePatcher<P>,
        progress_reporter: P,
    ) -> Self {
        Self {
            addon_dir,
            remote_patcher,
            progress_reporter,
        }
    }

    pub async fn apply(&self, diff: PackDiff) -> anyhow::Result<()> {
        let PackDiff {
            addon_diffs: node_diffs,
            ..
        } = diff;

        let total_dl_size: u64 = node_diffs.iter().map(|c| c.get_dl_size()).sum();
        log::info!(
            "Applying diff: {} change(s), {} bytes total",
            node_diffs.len(),
            total_dl_size
        );

        self.progress_reporter.start_for_download(total_dl_size);
        self.progress_reporter.report_message("Applying diff...");

        let res = stream::iter(node_diffs)
            .map(|diff| self.apply_node_diff(diff, RelPath::new()))
            .buffer_unordered(CONCURRENT_ADDON_SYNCS)
            .collect::<Vec<_>>()
            .await;

        self.progress_reporter.finish();

        let errors: Vec<anyhow::Error> = res.into_iter().filter_map(|r| r.err()).collect();

        if !errors.is_empty() {
            log::error!("{} error(s) occurred while applying diff", errors.len());
            let combined_error = errors
                .into_iter()
                .map(|e| format!("{:#}", e))
                .collect::<Vec<_>>()
                .join("\n");

            return Err(anyhow::anyhow!(
                "Errors occurred while applying diff:\n{}",
                combined_error
            ));
        }

        log::info!("Diff applied successfully");
        Ok(())
    }

    // Boxed because folders recurse into their children.
    fn apply_node_diff(
        &self,
        node_diff: NodeDiff,
        parent_path: RelPath,
    ) -> BoxFuture<'_, anyhow::Result<()>> {
        async move {
            match node_diff {
                NodeDiff::Created(node) => self.create_node(node, parent_path).await,
                NodeDiff::Deleted { name, .. } => self.delete_node(name, parent_path).await,
                NodeDiff::Modified(modification) => {
                    self.apply_modification(modification, parent_path).await
                }
                NodeDiff::None(_) => Ok(()),
            }
        }
        .boxed()
    }

    async fn delete_node(&self, name: String, parent_path: RelPath) -> anyhow::Result<()> {
        let path = parent_path.push(&name);
        let full_path = path.with_base_path(&self.addon_dir);

        log::debug!("Deleting {}", path);
        self.progress_reporter
            .report_message(&format!("Deleting {}", path));

        let metadata = fs::metadata(&full_path)
            .await
            .with_context(|| format!("Failed to inspect {:?} for deletion", full_path))?;

        if metadata.is_dir() {
            fs::remove_dir_all(&full_path)
                .await
                .context("Failed to delete directory")
        } else if metadata.is_file() {
            fs::remove_file(&full_path)
                .await
                .context("Failed to delete file")
        } else {
            unreachable!(
                "Path to delete is neither file nor directory: {:?}",
                full_path
            );
        }
    }

    // Boxed because folders recurse into their children.
    fn create_node(
        &self,
        node: IndexNode,
        parent_path: RelPath,
    ) -> BoxFuture<'_, anyhow::Result<()>> {
        async move {
            let path = parent_path.push(&node.name);

            match node.kind {
                NodeKind::File { length, .. } => {
                    log::debug!("Creating file {} ({} bytes)", path, length);
                    self.remote_patcher
                        .create_file(&path, &path.with_base_path(&self.addon_dir), length)
                        .await?;

                    self.progress_reporter
                        .report_message(&format!("Downloaded file {}", path));

                    Ok(())
                }
                NodeKind::Folder(children) => {
                    let dir_path = path.with_base_path(&self.addon_dir);
                    log::debug!("Creating directory {}", path);
                    self.progress_reporter
                        .report_message(&format!("Creating directory {}", path));
                    fs::create_dir(&dir_path)
                        .await
                        .with_context(|| format!("Failed to create directory {:?}", dir_path))?;

                    for child in children {
                        self.create_node(child, path.clone()).await?;
                    }
                    Ok(())
                }
            }
        }
        .boxed()
    }

    async fn apply_modification(
        &self,
        modification: NodeModification,
        parent_path: RelPath,
    ) -> anyhow::Result<()> {
        let path = parent_path.push(&modification.name);
        let file_path = path.with_base_path(&self.addon_dir);

        match modification.kind {
            ModifiedNodeKind::Folder(mut children) => {
                log::debug!("Applying modifications to folder {}", path);

                // We move deletes up because they COULD conflict with creates
                // This is mainly an issue on windows when a folder has the case of its name changed
                children.sort_by(|a, b| match (a, b) {
                    (NodeDiff::Deleted { .. }, NodeDiff::Deleted { .. }) => Ordering::Equal,
                    (NodeDiff::Deleted { .. }, _) => Ordering::Less,
                    (_, NodeDiff::Deleted { .. }) => Ordering::Greater,
                    _ => Ordering::Equal,
                });

                for child in children {
                    self.apply_node_diff(child, path.clone()).await?;
                }
            }
            ModifiedNodeKind::File { modification, .. } => {
                let dl_size = modification.get_dl_size();
                log::debug!("Patching file {} ({} bytes)", path, dl_size);
                self.progress_reporter
                    .report_message(&format!("Modifying file {}", path));
                self.remote_patcher
                    .patch_file(&path, &file_path, modification)
                    .await
                    .context(format!("Failed to patch: {}", path))?;
            }
        }

        Ok(())
    }
}
