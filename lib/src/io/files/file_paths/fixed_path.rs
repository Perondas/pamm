use crate::io::files::file_paths::rel_path::RelPath;

pub(crate) trait FixedFilePath {
    fn file_path() -> RelPath;
}