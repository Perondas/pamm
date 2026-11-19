use crate::domain::repo::repo_config::RepoConfig;
use crate::domain::repo::repo_version::RepoVersion;
use crate::domain::repo::server_config::ServerConfig;
use crate::infrastructure::fs::files::fs_writable::FixedFsWritable;
use crate::infrastructure::fs::files::name_consts::MEDIA_DIR_NAME;
use crate::infrastructure::fs::files::util::path_checks::is_dir;
use std::path::{Path, PathBuf};
use tokio::fs;

impl RepoConfig {
    /// Creates the repo directory and returns its path, `<dest_dir>/<repo name>`.
    /// The result is relative to whatever `dest_dir` is relative to — absolute if
    /// `dest_dir` is absolute, otherwise relative (usually to the process working
    /// directory).
    pub async fn init_server_repo_blank_on_fs(&self, dest_dir: &Path) -> anyhow::Result<PathBuf> {
        if !is_dir(dest_dir).await {
            anyhow::bail!("{} is not a directory", dest_dir.display());
        }

        let base_path = dest_dir.join(&self.name);

        if fs::try_exists(&base_path).await? {
            anyhow::bail!("Directory {} already exists", base_path.display());
        }

        fs::create_dir(&base_path).await?;
        fs::create_dir(base_path.join(MEDIA_DIR_NAME)).await?;
        self.write_fixed(&base_path).await?;
        RepoVersion::current().write_fixed(&base_path).await?;

        let server_config = ServerConfig::default();
        server_config.write_fixed(&base_path).await?;

        Ok(base_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::fs::files::file_names::fixed_file::FixedFile;
    use crate::support::test::test_utils::TestTempDir;
    use std::collections::HashSet;

    #[tokio::test]
    async fn test_init_blank_on_fs() {
        let temp_dir = TestTempDir::new("test_repo_init_blank");
        let base_path = temp_dir.path();

        let mut packs = HashSet::new();
        packs.insert("my_pack".to_string());

        let config = RepoConfig::new("my_repo".to_string(), "A test repo".to_string(), packs);

        // This should succeed
        let created_path = config
            .init_server_repo_blank_on_fs(base_path)
            .await
            .unwrap();

        assert!(created_path.exists());
        assert!(created_path.is_dir());
        assert_eq!(created_path.file_name().unwrap(), "my_repo");

        // Check if config was written
        let config_file = created_path.join(RepoConfig::file_name());
        assert!(config_file.exists());

        // Check if server config was written
        let server_config_file = created_path.join(ServerConfig::file_name());
        assert!(server_config_file.exists());

        // Calling it again should fail since it already exists
        let result = config.init_server_repo_blank_on_fs(base_path).await;
        assert!(result.is_err());
    }
}
