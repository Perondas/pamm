use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub(crate) fn clean_path(path: PathBuf) -> String {
    path.to_str()
        .expect("mods must be UTF-8")
        .strip_prefix("\\\\?\\")
        .unwrap()
        .to_string()
}

#[cfg(target_os = "linux")]
pub(crate) fn clean_path(path: PathBuf) -> String {
    path.to_str().expect("mods must be UTF-8").to_string()
}
