use crate::repo::repo_config::RepoConfig;
use crate::repo::repo_user_settings::RepoUserSettings;

pub trait KnownFile {
    fn file_name() -> &'static str;
}

#[macro_export]
macro_rules! known_file_name {
    ($ty:ty, $expr:expr) => {
        impl $crate::io::known_file::KnownFile for $ty {
            fn file_name() -> &'static str {
                $expr
            }
        }
    };
}

known_file_name!(RepoConfig, "repo.config.json");
known_file_name!(RepoUserSettings, "user.repo.settings.json");
