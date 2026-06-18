use crate::models::index::checksum_index::ChecksumIndex;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;
use crate::models::server::ServerConfig;

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
known_file_name!(ServerConfig, "server.config.json");

known_file_name!(ChecksumIndex, "checksum_index.pamm");
