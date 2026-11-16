use crate::domain::index::checksum_index::ChecksumIndex;
use crate::domain::pack::pack_config::PackConfig;
use crate::domain::pack::pack_user_settings::PackUserSettings;
use crate::domain::repo::repo_config::RepoConfig;
use crate::domain::repo::repo_user_settings::RepoUserSettings;
use crate::domain::repo::repo_version::RepoVersion;
use crate::domain::repo::server_config::ServerConfig;

pub trait FixedFile {
    fn file_name() -> &'static str;
}

#[macro_export]
macro_rules! known_file_name {
    ($ty:ty, $expr:expr) => {
        impl $crate::infrastructure::fs::files::file_names::fixed_file::FixedFile for $ty {
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
known_file_name!(RepoVersion, "version.pamm");
known_file_name!(PackConfig, "pack.config.json");
known_file_name!(PackUserSettings, "pack.settings.json");
