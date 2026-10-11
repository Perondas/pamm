use crate::handle::actions::build::build_pack::build_pack_inner;
use crate::handle::actions::build::materializer::Materializer;
use crate::handle::actions::build::{BuildOptions, BuildReport};
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::handle::server_repo_handle::ServerRepoHandle;
use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::files::file_paths::rel_path::RelPath;
use crate::io::files::name_consts::MEDIA_DIR_NAME;
use crate::io::fs::fs_writable::FixedFsWritable;
use crate::io::fs::util::path_checks::{is_dir, is_file};
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::models::pack::pack_config::PackConfig;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_version::RepoVersion;
use anyhow::Context;
use tokio::fs;

impl ServerRepoHandle {
    /// Build every pack listed in `repo.config.json` into `www/` (one
    /// `www/<pack>/` subtree each), plus the top-level `repo.config.json` and
    /// `version.pamm`. Removes stale pack subtrees from `www/` whose names are
    /// no longer in the repo config, as well as leftovers of the legacy (v1)
    /// flat layout.
    pub async fn build(
        &self,
        opts: BuildOptions,
        progress_reporter: impl ProgressReporter,
    ) -> anyhow::Result<BuildReport> {
        let www_path = self.get_www_path();
        fs::create_dir_all(&www_path)
            .await
            .with_context(|| format!("Failed to create www directory {:?}", www_path))?;

        let pack_names = self.get_config().packs.iter().collect::<Vec<_>>();
        let materializer = Materializer::new(opts.mode, &self.repo_path, &www_path);
        let mut report = BuildReport::from(&materializer);

        for pack_name in &pack_names {
            let inner_report =
                build_pack_inner(self, pack_name, opts, &materializer, &progress_reporter).await?;
            report = report + inner_report;
        }

        materializer
            .materialize(&RelPath::from_name(RepoConfig::file_name()))
            .await?;
        RepoVersion::current().write_fixed(&www_path).await?;

        // Prune stale entries in www/ that aren't part of the current repo config.
        let mut stale_removed = 0_usize;

        // Publish the repo-level media/ directory (icons/banners referenced by
        // repo and pack configs). materialize handles the recursion, symlinking
        // and pruning of stale entries inside www/media itself.
        let media_rel = RelPath::from_name(MEDIA_DIR_NAME);
        let media_src = media_rel.with_base_path(&self.repo_path);
        let has_media = is_dir(&media_src).await
            && fs::read_dir(&media_src)
                .await?
                .next_entry()
                .await?
                .is_some();
        if has_media {
            let media_report = materializer.materialize(&media_rel).await?;
            report = report + media_report;
        } else {
            // Source media/ was removed or emptied: drop a leftover www/media.
            let www_media = media_rel.with_base_path(&www_path);
            if fs::try_exists(&www_media).await? {
                fs::remove_dir_all(&www_media).await.with_context(|| {
                    format!("Failed to remove stale www media dir {:?}", www_media)
                })?;
                stale_removed += 1;
            }
        }

        let mut entries = fs::read_dir(&www_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if is_dir(entry.path()).await {
                // Legacy v1 addon dirs are always stale; a pack dir (identified
                // by its pack.config.json) is stale when the pack is gone.
                // www/media is live and must survive this loop — it matches
                // neither condition, keep it that way when tightening pruning.
                let is_legacy = name_str.ends_with("_pack_addons");
                let is_stale_pack_dir = is_file(entry.path().join(PackConfig::file_name())).await
                    && !pack_names.iter().any(|p| name_str == p.as_str());
                if is_legacy || is_stale_pack_dir {
                    fs::remove_dir_all(entry.path()).await.with_context(|| {
                        format!("Failed to remove stale www pack dir {:?}", entry.path())
                    })?;
                    stale_removed += 1;
                }
                continue;
            }

            // Legacy v1 root-level pack configs are always stale (v2 configs
            // live inside the pack dirs).
            if is_file(entry.path()).await && name_str.ends_with(".pack.config.json") {
                fs::remove_file(entry.path()).await.with_context(|| {
                    format!("Failed to remove stale www pack config {:?}", entry.path())
                })?;
                stale_removed += 1;
            }
        }

        report.stale_removed = stale_removed;

        Ok(report)
    }
}
