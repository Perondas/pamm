use crate::handle::actions::build::{BuildMode, BuildReport};
use crate::io::fs::util::symlink::create_or_recreate_symlink;
use crate::io::files::file_paths::rel_path::RelPath;
use anyhow::{Context, anyhow};
use std::fs;
use std::path::{Component, Path, PathBuf};

/// Per-build helper that places source files into the `www/` build output, either
/// as relative symlinks (default) or as copies.
pub(crate) struct Materializer<'a> {
    pub mode: BuildMode,
    src: &'a Path,
    dest: &'a Path,
}

impl<'a> Materializer<'a> {
    pub fn new(mode: BuildMode, src: &'a Path, dest: &'a Path) -> Self {
        Self { mode, src, dest }
    }

    /// Materialize `rel_path` under the same relative path on both sides.
    pub fn materialize(&self, rel_path: &RelPath) -> anyhow::Result<BuildReport> {
        self.materialize_from(rel_path, rel_path, &[])
    }

    /// Materialize the source at `src_rel` (relative to the source root) into
    /// `dest_rel` (relative to the destination root). Directory trees keep the
    /// same entry names below the top level. `top_level_excludes` names direct
    /// children of the top-level directory to skip entirely — they are neither
    /// materialized nor counted as live during stale pruning, so a leftover copy
    /// in the destination gets removed.
    pub fn materialize_from(
        &self,
        src_rel: &RelPath,
        dest_rel: &RelPath,
        top_level_excludes: &[&str],
    ) -> anyhow::Result<BuildReport> {
        let src_path = src_rel.with_base_path(self.src);

        if src_path.is_file() {
            self.place_file(src_rel, dest_rel)
                .with_context(|| format!("Failed to materialize file at {:?}", src_path))?;

            let mut report = BuildReport::from(self);
            report.files_materialized = 1;
            Ok(report)
        } else if src_path.is_dir() {
            self.materialize_dir(src_rel, dest_rel, top_level_excludes)
        } else {
            unreachable!("Source path {:?} is neither file nor directory", src_path);
        }
    }

    /// Materialize a single file. `source` must exist and be a file; `dest` is the
    /// path in www/ to create. Parent directories are created as needed.
    fn place_file(&self, src_rel: &RelPath, dest_rel: &RelPath) -> anyhow::Result<()> {
        if let Some(parent) = dest_rel.with_base_path(self.dest).parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create destination directory {:?}", parent))?;
        }

        match self.mode {
            BuildMode::Symlink => self.place_symlink(src_rel, dest_rel),
            BuildMode::Copy => self.place_copy(src_rel, dest_rel),
        }
    }

    fn materialize_dir(
        &self,
        src_rel: &RelPath,
        dest_rel: &RelPath,
        excludes: &[&str],
    ) -> anyhow::Result<BuildReport> {
        let src = src_rel.with_base_path(self.src);
        let dst = dest_rel.with_base_path(self.dest);

        fs::create_dir_all(&dst)
            .with_context(|| format!("Failed to create directory {:?}", dst))?;

        let mut report = BuildReport::default();

        // Remove stale entries in dst that are not present in src (excluded
        // entries count as absent, so leftovers of them in dst are pruned too).
        if dst.is_dir() {
            use std::collections::HashSet;

            let src_entries = fs::read_dir(&src)
                .with_context(|| format!("Failed to read dir {:?}", src))?
                .filter_map(Result::ok)
                .map(|e| e.file_name())
                .filter(|name| !excludes.iter().any(|excluded| name == excluded))
                .collect::<HashSet<_>>();

            for entry in fs::read_dir(&dst).with_context(|| format!("Failed to read dir {:?}", dst))? {
                let entry = entry?;
                if !src_entries.contains(&entry.file_name()) {
                    let path = entry.path();
                    if path.is_dir() {
                        fs::remove_dir_all(&path).with_context(|| format!("Failed to remove stale dir {:?}", path))?;
                    } else {
                        fs::remove_file(&path).with_context(|| format!("Failed to remove stale file {:?}", path))?;
                    }
                    report.stale_removed += 1;
                }
            }
        }


        for entry in fs::read_dir(&src).with_context(|| format!("Failed to read dir {:?}", src))? {
            let entry = entry?;
            let entry_path = entry.path();
            let entry_name = entry.file_name();

            if excludes.iter().any(|excluded| entry_name == *excluded) {
                continue;
            }

            let entry_name = entry_name.to_string_lossy();
            let src_rel = src_rel.push(&entry_name);
            let dest_rel = dest_rel.push(&entry_name);

            if entry_path.is_dir() {
                let res = self.materialize_dir(&src_rel, &dest_rel, &[])?;
                report = report + res;
            } else if entry_path.is_file() {
                self.place_file(&src_rel, &dest_rel)?;
                report.files_materialized += 1;
            } else {
                return Err(anyhow!(
                    "Unexpected non-file, non-directory entry at {:?}",
                    entry_path
                ));
            }
        }

        Ok(report)
    }

    fn place_symlink(&self, src_rel: &RelPath, dest_rel: &RelPath) -> anyhow::Result<()> {
        let dest = dest_rel.with_base_path(self.dest);
        let source = src_rel.with_base_path(self.src);

        let link_parent = dest
            .parent()
            .ok_or_else(|| anyhow!("symlink destination {:?} has no parent directory", dest))?;
        let target = relative_path(link_parent, &source).ok_or_else(|| {
            anyhow!(
                "Could not compute relative path from {:?} to {:?}",
                link_parent,
                source
            )
        })?;

        // If dest exists as a regular file/dir from a prior copy-mode build, clear
        // it so create_or_recreate_symlink (which refuses to clobber non-symlinks)
        // can place a fresh symlink. We own the entire www subtree, so this is safe.
        if let Ok(md) = fs::symlink_metadata(&dest)
            && !md.file_type().is_symlink()
        {
            if md.is_dir() {
                fs::remove_dir_all(&dest).with_context(|| {
                    format!("Failed to remove existing dir at {:?} before symlink", dest)
                })?;
            } else {
                fs::remove_file(&dest).with_context(|| {
                    format!(
                        "Failed to remove existing file at {:?} before symlink",
                        dest
                    )
                })?;
            }
        }

        create_or_recreate_symlink(&target, &dest).with_context(|| {
            format!(
                "Failed to create symlink at {:?} pointing to {:?}",
                dest, target
            )
        })
    }

    fn place_copy(&self, src_rel: &RelPath, dest_rel: &RelPath) -> anyhow::Result<()> {
        // If dest is an existing symlink (e.g. from an earlier symlink-mode build),
        // remove it before copying so we get a real file.

        let dest = dest_rel.with_base_path(self.dest);
        let source = src_rel.with_base_path(self.src);

        if fs::symlink_metadata(&dest)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
        {
            fs::remove_file(&dest).with_context(|| {
                format!("Failed to remove existing symlink {:?} before copy", dest)
            })?;
        }
        fs::copy(&source, &dest)
            .with_context(|| format!("Failed to copy {:?} -> {:?}", source, dest))?;
        Ok(())
    }
}

/// Compute a relative path from `from_dir` to `to`. Both paths should refer to
/// locations under a common ancestor (typically the repo root).
///
/// The returned path is always relative — relative to `from_dir`, e.g.
/// `../../foo_pack_addons/@addon/file.pbo` — regardless of whether the inputs
/// were absolute or relative (they just have to be both one or the other).
///
/// We do not canonicalize: callers want the symlink target to remain relative so
/// the whole repo directory stays portable.
pub fn relative_path(from_dir: &Path, to: &Path) -> Option<PathBuf> {
    let from: Vec<Component<'_>> = from_dir.components().collect();
    let to: Vec<Component<'_>> = to.components().collect();
    let common = from
        .iter()
        .zip(to.iter())
        .take_while(|(a, b)| a == b)
        .count();

    let up = from.len().checked_sub(common)?;
    let mut out = PathBuf::new();
    for _ in 0..up {
        out.push("..");
    }
    for c in &to[common..] {
        out.push(c.as_os_str());
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_path_sibling() {
        let from = Path::new("/repo/www/foo_pack_addons/@addon/sub");
        let to = Path::new("/repo/foo_pack_addons/@addon/sub/file.pbo");
        assert_eq!(
            relative_path(from, to).unwrap(),
            PathBuf::from("../../../../foo_pack_addons/@addon/sub/file.pbo")
        );
    }

    #[test]
    fn relative_path_same_dir() {
        let from = Path::new("/a/b");
        let to = Path::new("/a/b/file");
        assert_eq!(relative_path(from, to).unwrap(), PathBuf::from("file"));
    }

    #[test]
    fn relative_path_one_up() {
        let from = Path::new("/a/b/c");
        let to = Path::new("/a/b/file");
        assert_eq!(relative_path(from, to).unwrap(), PathBuf::from("../file"));
    }

    // www keeps the flat name while the source lives in a per-pack folder.
    #[test]
    fn relative_path_across_layouts() {
        let from = Path::new("/repo/www/foo_pack_addons/@addon");
        let to = Path::new("/repo/foo/addons/@addon/file.pbo");
        assert_eq!(
            relative_path(from, to).unwrap(),
            PathBuf::from("../../../foo/addons/@addon/file.pbo")
        );
    }
}
