use crate::handle::actions::build::build_pack::build_pack_inner;
use crate::handle::actions::build::materializer::Materializer;
use crate::handle::actions::build::{BuildOptions, BuildReport};
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::handle::server_repo_handle::ServerRepoHandle;
use crate::io::fs::fs_writable::KnownFSWritable;
use crate::io::known_file::KnownFile;
use crate::io::name_consts::PACK_CONFIG_FILE_NAME;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::io::rel_path::RelPath;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_version::RepoVersion;
use anyhow::Context;
use std::fs;

impl ServerRepoHandle {
    /// Build every pack listed in `repo.config.json` into `www/` (one
    /// `www/<pack>/` subtree each), plus the top-level `repo.config.json` and
    /// `version.pamm`. Removes stale pack subtrees from `www/` whose names are
    /// no longer in the repo config, as well as leftovers of the legacy (v1)
    /// flat layout.
    pub fn build(
        &self,
        opts: BuildOptions,
        progress_reporter: impl ProgressReporter,
    ) -> anyhow::Result<BuildReport> {
        let www_path = self.get_www_path();
        fs::create_dir_all(&www_path)
            .with_context(|| format!("Failed to create www directory {:?}", www_path))?;

        let pack_names = self.get_config().packs.iter().collect::<Vec<_>>();
        let mut materializer = Materializer::new(opts.mode, &self.repo_path, &www_path);
        let mut report = BuildReport::from(&materializer);

        for pack_name in &pack_names {
            let inner_report =
                build_pack_inner(self, pack_name, opts, &mut materializer, &progress_reporter)?;
            report = report + inner_report;
        }

        materializer.materialize(&RelPath::from_name(RepoConfig::file_name()))?;
        RepoVersion::current().write_to(&www_path)?;

        // Prune stale entries in www/ that aren't part of the current repo config.
        let mut stale_removed = 0_usize;
        for entry in fs::read_dir(&www_path)? {
            let entry = entry?;
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if entry.path().is_dir() {
                // Legacy v1 addon dirs are always stale; a pack dir (identified
                // by its pack.config.json) is stale when the pack is gone.
                let is_legacy = name_str.ends_with("_pack_addons");
                let is_stale_pack_dir = entry.path().join(PACK_CONFIG_FILE_NAME).is_file()
                    && !pack_names.iter().any(|p| name_str == p.as_str());
                if is_legacy || is_stale_pack_dir {
                    fs::remove_dir_all(entry.path()).with_context(|| {
                        format!("Failed to remove stale www pack dir {:?}", entry.path())
                    })?;
                    stale_removed += 1;
                }
                continue;
            }

            // Legacy v1 root-level pack configs are always stale (v2 configs
            // live inside the pack dirs).
            if entry.path().is_file() && name_str.ends_with(".pack.config.json") {
                fs::remove_file(entry.path()).with_context(|| {
                    format!("Failed to remove stale www pack config {:?}", entry.path())
                })?;
                stale_removed += 1;
            }
        }

        report.stale_removed = stale_removed;

        Ok(report)
    }
}
