use crate::at_pack_root;
use crate::io::files::file_paths::rel_path::RelPath;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::server_config::ServerConfig;

pub(crate) trait FixedFilePath {
    fn file_path() -> RelPath;
}

#[macro_export]
macro_rules! at_root {
    ($ty:ty) => {
        impl $crate::io::files::file_paths::fixed_path::FixedFilePath for $ty {
            fn file_path() -> RelPath {
                static_assertions::assert_impl_all!($ty: $crate::io::files::file_names::fixed_file::FixedFile);
                RelPath::new()
            }
        }
    };
}

at_root!(RepoConfig);
at_root!(ServerConfig);
