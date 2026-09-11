#[cfg(feature = "client")]
pub mod add_local_pack;
pub mod add_pack;
pub mod delete_pack;
#[cfg(feature = "client")]
pub mod remove_local_pack;
pub mod save_pack_settings;
pub mod update_pack;
pub mod update_repo_config;

use crate::handle::repo_handle::RepoHandle;
use crate::io::files::file_names::fixed_file::FixedFile;
#[cfg(feature = "client")]
use crate::io::files::file_paths::keyed_path::KeyedFilePath;
use crate::io::files::file_paths::self_identified_path::SelfIdentifiedFilePath;
use crate::io::files::name_consts::{
    CACHE_DB_DIR_NAME, INDEX_DIR_NAME, MEDIA_DIR_NAME, WWW_DIR_NAME,
};
use crate::io::fs::fs_writable::FixedFsWritable;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;
use crate::models::repo::repo_version::RepoVersion;
use crate::models::server_config::ServerConfig;

impl RepoHandle {
    pub(in crate::handle) fn write<T: FixedFsWritable + SelfIdentifiedFilePath>(
        &self,
        value: &T,
    ) -> anyhow::Result<()> {
        let path = value.file_path().with_base_path(&self.repo_path);
        value.write_fixed(&path)
    }

    #[cfg(feature = "client")]
    pub(in crate::handle) fn write_keyed<T: FixedFsWritable + KeyedFilePath>(
        &self,
        value: &T,
        key: &str,
    ) -> anyhow::Result<()> {
        let path = T::file_path(key).with_base_path(&self.repo_path);
        value.write_fixed(&path)
    }
}

/// Repo-root entries a pack folder must not shadow. The fixed file names are
/// included because a pack folder sits at the repo root alongside them.
fn reserved_root_names() -> [&'static str; 8] {
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
