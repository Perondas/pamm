use anyhow::Context;
use std::path::{Path, PathBuf};

pub(crate) fn canonicalize_and_clean_path<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    let p = path.as_ref();
    let canonicalized = p
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize {:#?}", p))?;
    clean_path(canonicalized)
}

#[cfg(target_os = "windows")]
pub(crate) fn clean_path(path: PathBuf) -> anyhow::Result<String> {
    Ok(path
        .to_str()
        .context("mods must be UTF-8")?
        // This is required as canonical paths on windows are prefixed with "\\?\" to indicate that they are in extended-length format.
        // Arma cannot handle this prefix, so we need to remove it.
        .strip_prefix("\\\\?\\")
        .context("got unexpectedly formatted windows path")?
        .to_string())
}

#[cfg(target_os = "linux")]
pub(crate) fn clean_path(path: PathBuf) -> anyhow::Result<String> {
    Ok(path.to_str().context("mods must be UTF-8")?.to_string())
}
