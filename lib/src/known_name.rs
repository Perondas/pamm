use crate::known_name_macro;
use crate::name_consts::{
    MANIFEST_FILE_NAME, PACK_CONFIG_FILE_NAME, REMOTE_FILE_NAME, REPO_CONFIG_FILE_NAME,
};
use crate::pack::config::pack_config::PackConfig;
use crate::pack::manifest::pack_manifest::PackManifest;
use crate::repo::remote_config::RemoteConfig;
use crate::repo::repo_config::RepoConfig;

pub trait KnownName {
    fn known_name() -> &'static str;
}

known_name_macro!(PackConfig, PACK_CONFIG_FILE_NAME);
known_name_macro!(PackManifest, MANIFEST_FILE_NAME);
known_name_macro!(RemoteConfig, REMOTE_FILE_NAME);
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
