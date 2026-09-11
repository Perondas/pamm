use crate::io::files::name_consts::MEDIA_DIR_NAME;
use crate::io::fs::fs_writable::FixedFsWritable;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_version::RepoVersion;
use std::fs;
use std::path::{Path, PathBuf};

impl RepoConfig {
    /// Creates the repo directory and returns its path, `<dest_dir>/<repo name>`.
    /// The result is relative to whatever `dest_dir` is relative to — absolute if
    /// `dest_dir` is absolute, otherwise relative (usually to the process working
    /// directory).
    pub fn init_blank_on_fs(&self, dest_dir: &Path) -> anyhow::Result<PathBuf> {
        if !dest_dir.is_dir() {
            anyhow::bail!("{} is not a directory", dest_dir.display());
        }

        let base_path = dest_dir.join(&self.name);

        if base_path.exists() {
            anyhow::bail!("Directory {} already exists", base_path.display());
        }

        fs::create_dir(&base_path)?;
        fs::create_dir(base_path.join(MEDIA_DIR_NAME))?;
        self.write_fixed(&base_path)?;
        RepoVersion::current().write_fixed(&base_path)?;

        Ok(base_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::files::file_names::fixed_file::FixedFile;
    use crate::util::test_utils::TestTempDir;
    use std::collections::HashSet;

    #[test]
    fn test_init_blank_on_fs() {
        let temp_dir = TestTempDir::new("test_repo_init_blank");
        let base_path = temp_dir.path();

        let mut packs = HashSet::new();
        packs.insert("my_pack".to_string());

        let config = RepoConfig::new("my_repo".to_string(), "A test repo".to_string(), packs);

        // This should succeed
        let created_path = config.init_blank_on_fs(base_path).unwrap();

        assert!(created_path.exists());
        assert!(created_path.is_dir());
        assert_eq!(created_path.file_name().unwrap(), "my_repo");

        // Check if config was written
        let config_file = created_path.join(RepoConfig::file_name());
        assert!(config_file.exists());

        // Calling it again should fail since it already exists
        let result = config.init_blank_on_fs(base_path);
        assert!(result.is_err());
    }
}
