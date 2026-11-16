use crate::domain::repo::repo_config::RepoConfig;
use crate::domain::repo::repo_user_settings::RepoUserSettings;
use crate::domain::repo::repo_version::RepoVersion;
use crate::domain::repo::server_config::ServerConfig;
use crate::infrastructure::fs::files::file_names::fixed_file::FixedFile;
use crate::infrastructure::fs::files::name_consts::{
    CACHE_DB_DIR_NAME, INDEX_DIR_NAME, MEDIA_DIR_NAME, WWW_DIR_NAME,
};

/// Repo-root entries a pack folder must not shadow. The fixed file names are
/// included because a pack folder sits at the repo root alongside them.
pub fn reserved_root_names() -> [&'static str; 8] {
    [
        WWW_DIR_NAME,
        MEDIA_DIR_NAME,
        CACHE_DB_DIR_NAME,
        INDEX_DIR_NAME,
        RepoConfig::file_name(),
        RepoUserSettings::file_name(),
        ServerConfig::file_name(),
        RepoVersion::file_name(),
    ]
}
