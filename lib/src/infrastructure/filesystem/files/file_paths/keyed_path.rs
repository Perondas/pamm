use crate::domain::pack::pack_config::PackConfig;
use crate::domain::pack::pack_user_settings::PackUserSettings;
use crate::infrastructure::filesystem::files::file_paths::rel_path::RelPath;

pub(crate) trait KeyedFilePath {
    fn file_path(key: &str) -> RelPath;
}

#[macro_export]
macro_rules! keyed_path {
    ($ty:ty, $format:expr) => {
        impl $crate::infrastructure::filesystem::files::file_paths::keyed_path::KeyedFilePath
            for $ty
        {
            fn file_path(
                key: &str,
            ) -> $crate::infrastructure::filesystem::files::file_paths::rel_path::RelPath {
                let path = format!($format, key);
                $crate::infrastructure::filesystem::files::file_paths::rel_path::RelPath::from_path(
                    &path,
                )
            }
        }
    };
}

keyed_path!(PackUserSettings, "{}");
keyed_path!(PackConfig, "{}");
