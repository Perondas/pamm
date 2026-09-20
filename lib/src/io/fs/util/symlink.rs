use anyhow::{Context, anyhow};
use std::fs::symlink_metadata;
use std::path::{Component, Path, PathBuf};

/// Create a symlink at `link` pointing to `target`. If `link` already exists as a
/// symlink, it is removed and recreated. If it exists as a regular file or directory,
/// an error is returned (we never silently clobber non-link state).
///
/// On Linux/macOS the symlink is created via [`std::os::unix::fs::symlink`].
/// On Windows we attempt [`std::os::windows::fs::symlink_dir`] if `target` is a
/// directory and [`std::os::windows::fs::symlink_file`] otherwise; both require
/// Developer Mode or admin privileges on Windows.
pub fn create_or_recreate_symlink(target: &Path, link: &Path) -> anyhow::Result<()> {
    log::trace!("Creating symlink: {:?} -> {:?}", link, target);
    if link.exists() || symlink_metadata(link).is_ok() {
        if symlink_metadata(link)?.file_type().is_symlink() {
            log::trace!("Removing existing symlink at {:?}", link);
            std::fs::remove_file(link)
                .context(anyhow!("Failed to remove existing symlink at {:?}", link))?;
        } else {
            return Err(anyhow!(
                "Path {:?} already exists and is not a symlink",
                link
            ));
        }
    }
    do_symlink(target, link).context("Failed to create symlink")
}

#[cfg(unix)]
fn do_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn do_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    if target.is_dir() {
        std::os::windows::fs::symlink_dir(target, link)
    } else {
        std::os::windows::fs::symlink_file(target, link)
    }
}

/// Compute a relative path from `from_dir` to `to`. Both paths have to be given
/// the same way -- both absolute, or both relative to the same base -- and free
/// of `.` and `..` segments.
///
/// The returned path is always relative, e.g. `../../@addon/file.pbo`. A relative
/// symlink target is what keeps a tree portable: it never names the prefix the
/// two paths share, so it keeps resolving after that tree is moved, or after a
/// sandbox mounts something else in its place.
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

    #[test]
    fn relative_path_walks_back_to_the_root_when_nothing_is_shared() {
        let from = Path::new("/home/user/games/pamm");
        let to = Path::new("/mnt/mods/repo");
        assert_eq!(
            relative_path(from, to).unwrap(),
            PathBuf::from("../../../../mnt/mods/repo")
        );
    }

    // The point of a relative target for the Linux launch: the link has to
    // resolve to the same directory once its prefix is remapped, which is what
    // Flatpak Steam's bind mount over `$HOME` does.
    #[test]
    fn a_relative_target_survives_a_remapped_prefix() {
        let host = relative_path(
            Path::new(
                "/home/user/.var/app/com.valvesoftware.Steam/.local/share/Steam/\
                 steamapps/common/Arma 3/pamm",
            ),
            Path::new("/home/user/.var/app/com.valvesoftware.Steam/pamm_repo"),
        );
        let sandbox = relative_path(
            Path::new("/home/user/.local/share/Steam/steamapps/common/Arma 3/pamm"),
            Path::new("/home/user/pamm_repo"),
        );

        assert_eq!(host, sandbox);
    }
}
