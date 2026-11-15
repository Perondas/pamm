use crate::domain::repo::repo_config::RepoConfig;
use crate::domain::repo::repo_user_settings::RepoUserSettings;
use crate::domain::repo::server_config::ServerConfig;
use crate::infrastructure::filesystem::files::file_paths::rel_path::RelPath;

pub(crate) trait FixedFilePath {
    fn file_path() -> RelPath;
}

#[macro_export]
macro_rules! at_root {
    ($ty:ty) => {
        impl $crate::infrastructure::filesystem::files::file_paths::fixed_path::FixedFilePath for $ty {
            fn file_path() -> RelPath {
                static_assertions::assert_impl_all!($ty: $crate::infrastructure::filesystem::files::file_names::fixed_file::FixedFile);
                RelPath::new()
            }
        }
    };
}

at_root!(RepoConfig);
at_root!(ServerConfig);
at_root!(RepoUserSettings);
