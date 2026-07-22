use crate::io::files::file_paths::rel_path::RelPath;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_user_settings::PackUserSettings;

pub(crate) trait KeyedFilePath {
    fn file_path(key: &str) -> RelPath;
}

#[macro_export]
macro_rules! key_is_folder {
    ($ty:ty) => {
        impl $crate::io::files::file_paths::keyed_path::KeyedFilePath for $ty {
            fn file_path(key: &str) ->  $crate::io::files::file_paths::RelPath {
                $crate::io::files::file_paths::RelPath::from_name(key)
            }
        }
    };
}

key_is_folder!(PackUserSettings);
key_is_folder!(PackConfig);
