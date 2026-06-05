use crate::handle::actions::build::materializer::Materializer;
use crate::handle::actions::build::{BuildOptions, BuildReport};
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::handle::server_repo_handle::ServerRepoHandle;
use crate::io::fs::pack::index_generator::IndexGenerator;
use crate::io::known_file::KnownFile;
use crate::io::name_consts::get_pack_addon_directory_name;
use crate::io::named_file::NamedFile;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::io::rel_path::RelPath;
use crate::models::pack::pack_config::PackConfig;
use crate::models::repo::repo_config::RepoConfig;
use anyhow::{Context, ensure};
use std::fs;

impl ServerRepoHandle {
    /// Build a single pack's `www/<pack>_pack_addons/` subtree. Also materializes
    /// `www/repo.config.json` and `www/<pack>.pack.config.json` so the published
    /// view stays in sync with the source's top-level configuration files.
    pub fn build_pack(
        &self,
        pack_name: &str,
        opts: BuildOptions,
        progress_reporter: &impl ProgressReporter,
    ) -> anyhow::Result<BuildReport> {
        ensure!(
            self.get_config().packs.contains(pack_name),
            "Pack '{}' not found in repo config",
            pack_name
        );

        let www_path = self.get_www_path();

        fs::create_dir_all(&www_path)?;

        let mut materializer = Materializer::new(opts.mode, &self.repo_path, &www_path);
        let report = build_pack_inner(self, pack_name, opts, &mut materializer, progress_reporter)?;

        // Top-level files: repo.config.json and this pack's config.
        materializer.materialize(&RelPath::from_name(RepoConfig::file_name()))?;
        materializer.materialize(&RelPath::from_name(&PackConfig::get_file_name(pack_name)))?;

        Ok(report)
    }
}

/// Core per-pack build routine, factored out so the whole-repo builder can call
/// it for each pack without re-materializing top-level files every time.
pub(super) fn build_pack_inner(
    handle: &ServerRepoHandle,
    pack_name: &str,
    opts: BuildOptions,
    materializer: &mut Materializer,
    progress_reporter: &impl ProgressReporter,
) -> anyhow::Result<BuildReport> {
    let www_path = handle.get_www_path();

    let addons_dir_name = get_pack_addon_directory_name(pack_name);

    let report = materializer.materialize(&RelPath::from_name(&addons_dir_name))?;

    // Index from source (uses the sled cache at <pack>_pack_addons/.cache/).
    let index_generator =
        IndexGenerator::from_handle(handle, pack_name, progress_reporter.clone())?;
    if opts.force_refresh {
        index_generator.clear_cache()?;
    }
    let pack_index = index_generator
        .index_addons()
        .context("Failed to index pack addons during build")?;

    // Write indexes into www (not next to source).
    pack_index
        .write_full_index_to_fs(&www_path)
        .context("Failed to write pack index into www")?;

    Ok(report)
}
