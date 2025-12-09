use crate::known_name_macro;
use crate::name_consts::{LOCAL_CONFIG_FILE_NAME, REPO_CONFIG_FILE_NAME};
use crate::repo::local_repo_config::LocalRepoConfig;
use crate::repo::repo_config::RepoConfig;

pub trait KnownName {
    fn known_name() -> &'static str;
}

known_name_macro!(LocalRepoConfig, LOCAL_CONFIG_FILE_NAME);
known_name_macro!(RepoConfig, REPO_CONFIG_FILE_NAME);

#[macro_export]
macro_rules! known_name_macro {
    ($ty:ty, $expr:expr) => {
        impl $crate::known_name::KnownName for $ty {
            fn known_name() -> &'static str {
                $expr
            }
        }
    };
}
