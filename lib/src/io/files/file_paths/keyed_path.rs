use crate::io::files::file_paths::rel_path::RelPath;

pub(crate) trait KeyedFilePath {
    fn file_path(key: &str) -> RelPath;
}
