use crate::handle::actions::build::materializer::Materializer;
use crate::handle::actions::build::{BuildOptions, BuildReport};
use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::handle::server_repo_handle::ServerRepoHandle;
use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::files::file_paths::rel_path::RelPath;
use crate::io::files::name_consts::{ADDONS_DIR_NAME, CACHE_DB_DIR_NAME};
use crate::io::fs::fs_writable::FixedFsWritable;
use crate::io::fs::pack::index_generator::IndexGenerator;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::models::pack::pack_config::PackConfig;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_version::RepoVersion;
use anyhow::{Context, ensure};

impl ServerRepoHandle {
    /// Build a single pack's `www/<pack>/` subtree. Also materializes
    /// `www/repo.config.json` and writes `www/version.pamm` so the published
    /// view stays in sync with the source's top-level configuration files.
    pub async fn build_pack(
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

        tokio::fs::create_dir_all(&www_path).await?;

        let materializer = Materializer::new(opts.mode, &self.repo_path, &www_path);
        let report =
            build_pack_inner(self, pack_name, opts, &materializer, progress_reporter).await?;

        // Top-level files: repo.config.json and the layout version marker.
        materializer
            .materialize(&RelPath::from_name(RepoConfig::file_name()))
            .await?;
        RepoVersion::current().write_fixed(&www_path).await?;

        Ok(report)
    }
}

/// Materialize `<pack>/pack.config.json` — source and `www/` share the
/// per-pack layout, so the relative path is the same on both sides.
pub(super) async fn materialize_pack_config(
    materializer: &Materializer,
    pack_name: &str,
) -> anyhow::Result<BuildReport> {
    materializer
        .materialize(&RelPath::from_name(pack_name).push(PackConfig::file_name()))
        .await
}

/// Core per-pack build routine, factored out so the whole-repo builder can call
/// it for each pack without re-materializing top-level files every time.
pub(super) async fn build_pack_inner(
    handle: &ServerRepoHandle,
    pack_name: &str,
    opts: BuildOptions,
    materializer: &Materializer,
    progress_reporter: &impl ProgressReporter,
) -> anyhow::Result<BuildReport> {
    let mut config = handle.get_pack(pack_name).await?;

    let www_path = handle.get_www_path();

    // Source `<pack>/addons/` maps onto the same path in `www/`. The sled
    // cache inside the source addons dir stays out of the published tree.
    let addons_rel = RelPath::from_name(pack_name).push(ADDONS_DIR_NAME);
    let report = materializer
        .materialize_from(&addons_rel, &addons_rel, &[CACHE_DB_DIR_NAME])
        .await?;

    materialize_pack_config(materializer, pack_name).await?;

    // Index from source (uses the sled cache at <pack>/addons/.cache/).
    let index_generator =
        IndexGenerator::from_handle(handle, pack_name, progress_reporter.clone()).await?;
    if opts.force_refresh {
        index_generator.clear_cache().await?;
    }
    let pack_index = index_generator
        .index_addons()
        .await
        .context("Failed to index pack addons during build")?;

    // Write indexes into www (not next to source).
    pack_index
        .write_full_index_to_fs(&www_path)
        .await
        .context("Failed to write pack index into www")?;

    // Update the pack config
    for index in &pack_index.addons {
        config.addons.entry(index.name.clone()).or_default();
    }

    config
        .addons
        .retain(|name, _| pack_index.addons.iter().any(|i| i.name == *name));

    handle.write(&config).await?;

    Ok(report)
}
