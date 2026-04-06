use std::fs;
use std::path::{Path, PathBuf};

pub struct TestTempDir(pub PathBuf);

impl TestTempDir {
    pub fn new(key: &str) -> Self {
        let temp_path = std::env::temp_dir().join(key);
        if temp_path.exists() {
            fs::remove_dir_all(&temp_path).ok();
        }

        fs::create_dir_all(&temp_path).ok();

        Self(temp_path)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestTempDir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).ok();
    }
}
