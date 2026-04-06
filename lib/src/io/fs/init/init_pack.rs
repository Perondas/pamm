use crate::io::fs::fs_writable::{IdentifiableFSWritable, NamedFSWritable};
use crate::io::name_consts::{INDEX_DIR_NAME, get_pack_addon_directory_name};
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_user_settings::PackUserSettings;
use std::fs;
use std::path::Path;

impl PackConfig {
    pub fn init_blank_on_fs(&self, parent_dir: &Path) -> anyhow::Result<()> {
        if !parent_dir.is_dir() {
            anyhow::bail!("{} is not a directory", parent_dir.display());
        }

        let addon_dir_name = get_pack_addon_directory_name(&self.name);

        fs::create_dir(parent_dir.join(&addon_dir_name))?;

        let index_dir = parent_dir.join(&addon_dir_name).join(INDEX_DIR_NAME);

        fs::create_dir(&index_dir)?;

        let settings = PackUserSettings::default();

        settings.write_to_named(parent_dir, &self.name)?;

        self.write_to(parent_dir)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_utils::TestTempDir;

    #[test]
    fn test_init_blank_on_fs() {
        let temp = TestTempDir::new("pamm_test_init_blank");

        let config = PackConfig {
            name: "test_pack".to_string(),
            description: "test pack".to_string(),
            client_params: vec![],
            servers: vec![],
            parent: None,
            addons: Default::default(),
        };

        config.init_blank_on_fs(temp.path()).unwrap();

        let addon_dir_name = get_pack_addon_directory_name(&config.name);
        let addon_dir = temp.path().join(&addon_dir_name);
        assert!(addon_dir.is_dir());

        let index_dir = addon_dir.join(INDEX_DIR_NAME);
        assert!(index_dir.is_dir());
    }

    #[test]
    fn test_init_blank_on_fs_not_dir() {
        let temp = TestTempDir::new("pamm_test_init_blank_not_dir");

        let config = PackConfig {
            name: "test_pack".to_string(),
            description: "test pack".to_string(),
            client_params: vec![],
            servers: vec![],
            parent: None,
            addons: Default::default(),
        };

        let file = temp.path().join("not_a_dir.txt");
        fs::write(&file, b"test").unwrap();

        let result = config.init_blank_on_fs(&file);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("{} is not a directory", file.display())
        );
    }
}
