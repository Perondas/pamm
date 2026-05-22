use anyhow::{Context, anyhow};
use std::fs::symlink_metadata;
use std::path::Path;

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
    // For Windows we need to know whether the target is a dir or file.
    // Resolve the target relative to the link's parent if it's relative.
    let resolved = if target.is_absolute() {
        target.to_path_buf()
    } else if let Some(parent) = link.parent() {
        parent.join(target)
    } else {
        target.to_path_buf()
    };

    if resolved.is_dir() {
        std::os::windows::fs::symlink_dir(target, link)
    } else {
        std::os::windows::fs::symlink_file(target, link)
    }
}
