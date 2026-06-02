use crate::io::fs::fs_writable::{IdentifiableFSWritable, NamedFSWritable};
use crate::io::name_consts::{INDEX_DIR_NAME, get_pack_addon_directory_name};
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_user_settings::PackUserSettings;
use std::fs;
use std::path::Path;

impl PackConfig {
    /// Lay out a new pack on the **server**: create the source addon directory and
    /// write the pack config. No client-only artifacts (`<pack>.pack.settings.json`)
    /// and no `indexes/` directory — indexes are produced into `www/` by the build.
    pub fn init_source_on_fs(&self, parent_dir: &Path) -> anyhow::Result<()> {
        if !parent_dir.is_dir() {
            anyhow::bail!("{} is not a directory", parent_dir.display());
        }

        let addon_dir_name = get_pack_addon_directory_name(&self.name);
        fs::create_dir(parent_dir.join(&addon_dir_name))?;

        self.write_to(parent_dir)
    }

    /// Lay out a new pack on the **client**: create the addon directory and its
    /// `indexes/` subdirectory, write the default user settings, and write the
    /// (downloaded) pack config.
    pub fn init_client_on_fs(&self, parent_dir: &Path) -> anyhow::Result<()> {
        if !parent_dir.is_dir() {
            anyhow::bail!("{} is not a directory", parent_dir.display());
        }

        let addon_dir_name = get_pack_addon_directory_name(&self.name);
        fs::create_dir(parent_dir.join(&addon_dir_name))?;

        let index_dir = parent_dir.join(&addon_dir_name).join(INDEX_DIR_NAME);
        fs::create_dir(&index_dir)?;

        let settings = PackUserSettings::default();
        settings.write_to_named(parent_dir, &self.name)?;

        self.write_to(parent_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_utils::TestTempDir;

    fn test_pack_config() -> PackConfig {
        PackConfig {
            name: "test_pack".to_string(),
            description: "test pack".to_string(),
            client_params: vec![],
            servers: vec![],
            parent: None,
            addons: Default::default(),
        }
    }

    #[test]
    fn test_init_client_on_fs() {
        let temp = TestTempDir::new("pamm_test_init_client");

        let config = test_pack_config();
        config.init_client_on_fs(temp.path()).unwrap();

        let addon_dir_name = get_pack_addon_directory_name(&config.name);
        let addon_dir = temp.path().join(&addon_dir_name);
        assert!(addon_dir.is_dir());

        let index_dir = addon_dir.join(INDEX_DIR_NAME);
        assert!(index_dir.is_dir());

        let settings_file = temp.path().join(format!("{}.pack.settings.json", config.name));
        assert!(settings_file.is_file(), "client layout writes pack settings");
    }

    #[test]
    fn test_init_source_on_fs_omits_client_artifacts() {
        let temp = TestTempDir::new("pamm_test_init_source");

        let config = test_pack_config();
        config.init_source_on_fs(temp.path()).unwrap();

        let addon_dir_name = get_pack_addon_directory_name(&config.name);
        let addon_dir = temp.path().join(&addon_dir_name);
        assert!(addon_dir.is_dir());

        // Server layout must NOT create the client-only settings file...
        let settings_file = temp.path().join(format!("{}.pack.settings.json", config.name));
        assert!(
            !settings_file.exists(),
            "server layout must not write pack settings"
        );

        // ...nor an indexes/ directory next to the source (indexes live in www/).
        let index_dir = addon_dir.join(INDEX_DIR_NAME);
        assert!(
            !index_dir.exists(),
            "server layout must not create indexes/ next to source"
        );
    }

    #[test]
    fn test_init_source_on_fs_not_dir() {
        let temp = TestTempDir::new("pamm_test_init_source_not_dir");

        let config = test_pack_config();

        let file = temp.path().join("not_a_dir.txt");
        fs::write(&file, b"test").unwrap();

        let result = config.init_source_on_fs(&file);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("{} is not a directory", file.display())
        );
    }
}
