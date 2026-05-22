use crate::handle::actions::build::BuildMode;
use crate::io::fs::util::symlink::create_or_recreate_symlink;
use anyhow::{Context, anyhow};
use std::fs;
use std::path::{Component, Path, PathBuf};

/// Per-build helper that places source files into the `www/` build output, either
/// as relative symlinks (default) or as copies. On Windows, if the first symlink
/// fails with a privilege error, switches the whole build to copy mode.
pub struct Materializer {
    #[cfg_attr(not(windows), allow(dead_code))]
    requested: BuildMode,
    actual: BuildMode,
}

impl Materializer {
    pub fn new(mode: BuildMode) -> Self {
        Self {
            requested: mode,
            actual: mode,
        }
    }

    pub fn actual_mode(&self) -> BuildMode {
        self.actual
    }

    /// Materialize a single file. `source` must exist and be a file; `dest` is the
    /// path in www/ to create. Parent directories are created as needed.
    pub fn place_file(&mut self, source: &Path, dest: &Path) -> anyhow::Result<()> {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create destination directory {:?}", parent)
            })?;
        }

        match self.actual {
            BuildMode::Symlink => self.place_symlink(source, dest),
            BuildMode::Copy => self.place_copy(source, dest),
        }
    }

    fn place_symlink(&mut self, source: &Path, dest: &Path) -> anyhow::Result<()> {
        let link_parent = dest.parent().ok_or_else(|| {
            anyhow!("symlink destination {:?} has no parent directory", dest)
        })?;
        let target = relative_path(link_parent, source).ok_or_else(|| {
            anyhow!(
                "Could not compute relative path from {:?} to {:?}",
                link_parent,
                source
            )
        })?;

        // If dest exists as a regular file/dir from a prior copy-mode build, clear
        // it so create_or_recreate_symlink (which refuses to clobber non-symlinks)
        // can place a fresh symlink. We own the entire www subtree, so this is safe.
        if let Ok(md) = fs::symlink_metadata(dest)
            && !md.file_type().is_symlink()
        {
            if md.is_dir() {
                fs::remove_dir_all(dest).with_context(|| {
                    format!("Failed to remove existing dir at {:?} before symlink", dest)
                })?;
            } else {
                fs::remove_file(dest).with_context(|| {
                    format!("Failed to remove existing file at {:?} before symlink", dest)
                })?;
            }
        }

        match create_or_recreate_symlink(&target, dest) {
            Ok(()) => Ok(()),
            #[cfg(windows)]
            Err(e) if self.requested == BuildMode::Symlink => {
                log::warn!(
                    "Symlink creation failed ({}); falling back to copy mode for the rest of the build",
                    e
                );
                self.actual = BuildMode::Copy;
                self.place_copy(source, dest)
            }
            Err(e) => Err(e),
        }
    }

    fn place_copy(&self, source: &Path, dest: &Path) -> anyhow::Result<()> {
        // If dest is an existing symlink (e.g. from an earlier symlink-mode build),
        // remove it before copying so we get a real file.
        if fs::symlink_metadata(dest)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
        {
            fs::remove_file(dest).with_context(|| {
                format!("Failed to remove existing symlink {:?} before copy", dest)
            })?;
        }
        fs::copy(source, dest).with_context(|| {
            format!("Failed to copy {:?} -> {:?}", source, dest)
        })?;
        Ok(())
    }
}

/// Compute a relative path from `from_dir` to `to`. Both paths should refer to
/// locations under a common ancestor (typically the repo root).
///
/// We do not canonicalize: callers want the symlink target to remain relative so
/// the whole repo directory stays portable.
pub fn relative_path(from_dir: &Path, to: &Path) -> Option<PathBuf> {
    let from: Vec<Component<'_>> = from_dir.components().collect();
    let to: Vec<Component<'_>> = to.components().collect();
    let common = from.iter().zip(to.iter()).take_while(|(a, b)| a == b).count();

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
}
