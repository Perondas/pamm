use crate::handle::actions::build::materializer::Materializer;
use crate::handle::actions::build::{BuildOptions, PackBuildReport};
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::handle::server_repo_handle::ServerRepoHandle;
use crate::io::fs::pack::index_generator::IndexGenerator;
use crate::io::known_file::KnownFile;
use crate::io::name_consts::{CACHE_DB_DIR_NAME, INDEX_DIR_NAME, get_pack_addon_directory_name};
use crate::io::named_file::NamedFile;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::models::pack::pack_config::PackConfig;
use crate::models::repo::repo_config::RepoConfig;
use anyhow::{Context, anyhow, ensure};
use std::fs;
use std::path::Path;

impl ServerRepoHandle {
    /// Build a single pack's `www/<pack>_pack_addons/` subtree. Also materializes
    /// `www/repo.config.json` and `www/<pack>.pack.config.json` so the published
    /// view stays in sync with the source's top-level configuration files.
    pub fn build_pack(
        &self,
        pack_name: &str,
        opts: BuildOptions,
        progress_reporter: &impl ProgressReporter,
    ) -> anyhow::Result<PackBuildReport> {
        ensure!(
            self.get_config().packs.contains(pack_name),
            "Pack '{}' not found in repo config",
            pack_name
        );

        let mut materializer = Materializer::new(opts.mode);
        let report = build_pack_inner(self, pack_name, opts, &mut materializer, progress_reporter)?;

        // Top-level files: repo.config.json and this pack's config.
        materialize_top_level(
            self.get_repo_path(),
            self.www_path(),
            RepoConfig::file_name(),
            &mut materializer,
        )?;
        materialize_top_level(
            self.get_repo_path(),
            self.www_path(),
            &PackConfig::get_file_name(pack_name),
            &mut materializer,
        )?;

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
) -> anyhow::Result<PackBuildReport> {
    let addons_dir_name = get_pack_addon_directory_name(pack_name);
    let source_pack_dir = handle.get_repo_path().join(&addons_dir_name);
    ensure!(
        source_pack_dir.is_dir(),
        "Source pack directory not found: {:?}",
        source_pack_dir
    );

    let www_pack_dir = handle.www_path().join(&addons_dir_name);

    // Wipe the target pack subtree before each build. This handles stale entries
    // in both symlink and copy modes uniformly and keeps the implementation simple.
    if www_pack_dir.exists() {
        fs::remove_dir_all(&www_pack_dir).with_context(|| {
            format!(
                "Failed to clear stale www pack directory {:?}",
                www_pack_dir
            )
        })?;
    }
    fs::create_dir_all(&www_pack_dir)
        .with_context(|| format!("Failed to create www pack directory {:?}", www_pack_dir))?;

    // Index from source (uses the sled cache at <pack>_pack_addons/.cache/).
    let index_generator =
        IndexGenerator::from_handle(handle, pack_name, progress_reporter.clone())?;
    if opts.force_refresh {
        index_generator.clear_cache()?;
    }
    let pack_index = index_generator
        .index_addons()
        .context("Failed to index pack addons during build")?;

    // Materialize each top-level entry under <pack>_pack_addons/ that should be
    // exposed. We deliberately skip `.cache/` (sled DB) and `indexes/` (those are
    // generated into www separately).
    let mut report = PackBuildReport {
        pack_name: pack_name.to_string(),
        addons_materialized: 0,
        files_materialized: 0,
        stale_removed: 0,
        mode: opts.mode,
    };

    for entry in fs::read_dir(&source_pack_dir)
        .with_context(|| format!("Failed to read source pack dir {:?}", source_pack_dir))?
    {
        let entry = entry?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str == CACHE_DB_DIR_NAME || name_str == INDEX_DIR_NAME {
            continue;
        }

        let src = entry.path();
        let dst = www_pack_dir.join(&name);

        if src.is_dir() {
            materialize_dir(&src, &dst, materializer, &mut report)?;
            report.addons_materialized += 1;
        } else if src.is_file() {
            materializer.place_file(&src, &dst)?;
            report.files_materialized += 1;
        }
    }

    // Write indexes into www (not next to source).
    pack_index
        .write_full_index_to_fs(handle.www_path())
        .context("Failed to write pack index into www")?;

    report.mode = opts.mode;
    Ok(report)
}

fn materialize_dir(
    src: &Path,
    dst: &Path,
    materializer: &mut Materializer,
    report: &mut PackBuildReport,
) -> anyhow::Result<()> {
    fs::create_dir_all(dst).with_context(|| format!("Failed to create directory {:?}", dst))?;

    for entry in fs::read_dir(src).with_context(|| format!("Failed to read dir {:?}", src))? {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_dst = dst.join(entry.file_name());

        if entry_path.is_dir() {
            materialize_dir(&entry_path, &entry_dst, materializer, report)?;
        } else if entry_path.is_file() {
            materializer.place_file(&entry_path, &entry_dst)?;
            report.files_materialized += 1;
        } else {
            return Err(anyhow!(
                "Unexpected non-file, non-directory entry at {:?}",
                entry_path
            ));
        }
    }

    Ok(())
}

pub(super) fn materialize_top_level(
    repo_path: &Path,
    www_path: &Path,
    file_name: &str,
    materializer: &mut Materializer,
) -> anyhow::Result<()> {
    let src = repo_path.join(file_name);
    let dst = www_path.join(file_name);
    ensure!(
        src.is_file(),
        "Expected top-level file {:?} to exist before build",
        src
    );
    fs::create_dir_all(www_path)
        .with_context(|| format!("Failed to create www directory {:?}", www_path))?;
    materializer.place_file(&src, &dst)
}
