use crate::handle::actions::build::build_pack::{build_pack_inner, materialize_top_level};
use crate::handle::actions::build::materializer::Materializer;
use crate::handle::actions::build::{BuildOptions, BuildReport, PackBuildReport};
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::handle::server_repo_handle::ServerRepoHandle;
use crate::io::known_file::KnownFile;
use crate::io::name_consts::get_pack_addon_directory_name;
use crate::io::named_file::NamedFile;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::io::rel_path::RelPath;
use crate::models::pack::pack_config::PackConfig;
use crate::models::repo::repo_config::RepoConfig;
use anyhow::Context;
use std::collections::HashSet;
use std::fs;

impl ServerRepoHandle {
    /// Build every pack listed in `repo.config.json` into `www/`, plus the
    /// top-level `repo.config.json` and per-pack `<pack>.pack.config.json` files.
    /// Removes any stale pack subtrees or pack configs from `www/` whose names
    /// are no longer in the repo config.
    pub fn build(
        &self,
        opts: BuildOptions,
        progress_reporter: impl ProgressReporter,
    ) -> anyhow::Result<BuildReport> {
        let www_path = self.get_www_path();
        fs::create_dir_all(&www_path)
            .with_context(|| format!("Failed to create www directory {:?}", &www_path))?;

        let pack_names = self.get_config().packs.iter().collect::<Vec<_>>();
        let mut materializer = Materializer::new(opts.mode, &self.repo_path, &www_path);
        let mut reports = Vec::with_capacity(pack_names.len());

        for pack_name in &pack_names {
            let report =
                build_pack_inner(self, pack_name, opts, &mut materializer, &progress_reporter)?;
            reports.push(report);
        }

        materializer.place_file(&RelPath::new().push(RepoConfig::file_name()))?;

        // Top-level files.
        materialize_top_level(RepoConfig::file_name(), &mut materializer)?;
        for pack_name in &pack_names {
            materialize_top_level(&PackConfig::get_file_name(pack_name), &mut materializer)?;
        }

        // Prune stale entries in www/ that aren't part of the current repo config.
        let expected_pack_dirs: HashSet<String> = pack_names
            .iter()
            .map(|p| get_pack_addon_directory_name(p))
            .collect();
        let expected_pack_configs: HashSet<String> = pack_names
            .iter()
            .map(|p| PackConfig::get_file_name(p))
            .collect();

        let mut stale_removed = 0_usize;
        for entry in fs::read_dir(&www_path)? {
            let entry = entry?;
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if entry.path().is_dir() && name_str.ends_with("_pack_addons") {
                if !expected_pack_dirs.contains(name_str.as_ref()) {
                    fs::remove_dir_all(entry.path()).with_context(|| {
                        format!("Failed to remove stale www pack dir {:?}", entry.path())
                    })?;
                    stale_removed += 1;
                }
                continue;
            }

            if entry.path().is_file()
                && name_str.ends_with(".pack.config.json")
                && !expected_pack_configs.contains(name_str.as_ref())
            {
                fs::remove_file(entry.path()).with_context(|| {
                    format!("Failed to remove stale www pack config {:?}", entry.path())
                })?;
                stale_removed += 1;
            }
        }

        if let Some(last) = reports.last_mut() {
            last.stale_removed += stale_removed;
        }

        for r in &mut reports {
            r.mode = opts.mode;
        }

        Ok(BuildReport { packs: reports })
    }
}
