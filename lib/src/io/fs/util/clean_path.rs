use std::path::PathBuf;

//TODO: Probably add a "canonicalize_and_clean_path" function that does both in one step, since we always do both together.

#[cfg(target_os = "windows")]
pub(crate) fn clean_path(path: PathBuf) -> String {
    path.to_str()
        .expect("mods must be UTF-8")
        // This is required as canonical paths on windows are prefixed with "\\?\" to indicate that they are in extended-length format.
        // Arma cannot handle this prefix, so we need to remove it.
        .strip_prefix("\\\\?\\")
        .expect("got unexpectedly formatted windows path")
        .to_string()
}

#[cfg(target_os = "linux")]
pub(crate) fn clean_path(path: PathBuf) -> String {
    path.to_str().expect("mods must be UTF-8").to_string()
}
