use crate::io::files::file_paths::keyed_path::KeyedFilePath;
use crate::io::files::file_paths::rel_path::RelPath;

pub(crate) trait SelfIdentifiedFilePath {
    fn file_path(&self) -> RelPath;
}
