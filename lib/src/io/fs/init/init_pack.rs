use crate::io::fs::fs_writable::FixedFsWritable;
use crate::io::files::name_consts::{ADDONS_DIR_NAME, INDEX_DIR_NAME, WWW_DIR_NAME};
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_user_settings::PackUserSettings;
use std::fs;
use std::path::Path;

impl PackConfig {
    /// Lay out a new pack on the **server**: create the pack folder with its
    /// `addons/` directory and write the pack config into it. No client-only
    /// artifacts (pack settings) and no `indexes/` directory — indexes are
    /// produced into `www/` by the build.
    pub fn init_source_on_fs(&self, parent_dir: &Path) -> anyhow::Result<()> {
        if !parent_dir.is_dir() {
            anyhow::bail!("{} is not a directory", parent_dir.display());
        }
        if self.name == WWW_DIR_NAME {
            anyhow::bail!(
                "Pack must not be named '{}': its folder would collide with the build output",
                WWW_DIR_NAME
            );
        }

        let addon_dir = parent_dir.join(&self.name);


        fs::create_dir_all(addon_dir.join(ADDONS_DIR_NAME))?;

        self.write_fixed(addon_dir)
    }

    /// Lay out a new pack on the **client**: create the pack folder with its
    /// `addons/` and `indexes/` directories, write the default user settings,
    /// and write the (downloaded) pack config.
    pub fn init_client_on_fs(&self, parent_dir: &Path) -> anyhow::Result<()> {
        if !parent_dir.is_dir() {
            anyhow::bail!("{} is not a directory", parent_dir.display());
        }

        let addon_dir = parent_dir.join(&self.name);

        fs::create_dir_all(addon_dir.join(ADDONS_DIR_NAME))?;
        fs::create_dir_all(addon_dir.join(INDEX_DIR_NAME))?;

        let settings = PackUserSettings::default();
        settings.write_fixed(&addon_dir)?;

        self.write_fixed(addon_dir)
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
            parent: None,
            addons: Default::default(),
        }
    }

    #[test]
    fn test_init_client_on_fs() {
        let temp = TestTempDir::new("pamm_test_init_client");

        let config = test_pack_config();
        config.init_client_on_fs(temp.path()).unwrap();

        let pack_dir = temp.path().join(&config.name);
        assert!(pack_dir.join("addons").is_dir());
        assert!(pack_dir.join("indexes").is_dir());
        assert!(pack_dir.join("pack.config.json").is_file());
        assert!(
            pack_dir.join("pack.settings.json").is_file(),
            "client layout writes pack settings"
        );
    }

    #[test]
    fn test_init_source_on_fs_uses_per_pack_layout() {
        let temp = TestTempDir::new("pamm_test_init_source");

        let config = test_pack_config();
        config.init_source_on_fs(temp.path()).unwrap();

        let pack_dir = temp.path().join(&config.name);
        assert!(pack_dir.join("addons").is_dir());
        assert!(pack_dir.join("pack.config.json").is_file());

        // No flat-layout artifacts at the repo root.
        assert!(
            !temp
                .path()
                .join(format!("{}_pack_addons", config.name))
                .exists()
        );
        assert!(
            !temp
                .path()
                .join(format!("{}.pack.config.json", config.name))
                .exists()
        );

        // Server layout must NOT create the client-only settings file...
        assert!(
            !pack_dir.join("pack.settings.json").exists(),
            "server layout must not write pack settings"
        );

        // ...nor an indexes/ directory (indexes live in www/).
        assert!(
            !pack_dir.join("indexes").exists(),
            "server layout must not create indexes/ next to source"
        );
    }

    #[test]
    fn test_init_source_on_fs_rejects_www_pack_name() {
        let temp = TestTempDir::new("pamm_test_init_source_www");

        let mut config = test_pack_config();
        config.name = "www".to_string();

        let err = config
            .init_source_on_fs(temp.path())
            .unwrap_err()
            .to_string();
        assert!(err.contains("www"), "unexpected error: {}", err);
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
