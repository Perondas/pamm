use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::io::name_consts::get_pack_addon_directory_name;
use log::{debug, trace};
use std::path::PathBuf;

impl<T> GetOptionalsPaths for T
where
    T: GetPack + GetRepoInfo,
{
    fn get_optional_paths(&self, pack_name: &str) -> anyhow::Result<Vec<PathBuf>> {
        let (config, settings) = self.get_pack_with_settings(pack_name)?;

        let mut res = Vec::new();

        let addon_dir = self
            .get_repo_path()
            .join(get_pack_addon_directory_name(&config.name));

        for optional in &settings.enabled_optionals {
            if config
                .addons
                .get(optional)
                .is_some_and(|addon| addon.is_optional)
            {
                trace!(
                    "Optional addon '{}' is enabled for pack '{}'",
                    optional, config.name
                );
                let optional_path = addon_dir.join(optional);
                res.push(optional_path);
            }
        }

        debug!(
            "Optional addon paths for pack '{}': {:#?}",
            config.name, res
        );

        let mut others = if let Some(parent) = config.parent {
            self.get_optional_paths(&parent)?
        } else {
            vec![]
        };

        debug!(
            "Parent optional addon paths for pack '{}': {:#?}",
            config.name, others
        );

        res.append(&mut others);

        Ok(res)
    }
}

pub trait GetOptionalsPaths {
    fn get_optional_paths(&self, pack_name: &str) -> anyhow::Result<Vec<PathBuf>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handle::mock_handle::{MockHandle, MockHandleExt};
    use std::path::PathBuf;

    #[test]
    fn test_get_optional_paths() {
        let mut mock = MockHandle::new();

        mock.mock_pack(
            "test_pack",
            None,
            &["@opt_addon"],
            &["@req_addon"],
            &["@opt_addon", "@req_addon"],
        );

        mock.expect_get_repo_path()
            .return_const(PathBuf::from("/repo"));

        let paths = mock.get_optional_paths("test_pack").unwrap();

        assert_eq!(paths.len(), 1);
        assert_eq!(
            paths[0],
            PathBuf::from("/repo")
                .join(get_pack_addon_directory_name("test_pack"))
                .join("@opt_addon")
        );
    }

    #[test]
    fn test_get_optional_paths_with_parent() {
        let mut mock = MockHandle::new();

        mock.mock_pack(
            "child_pack",
            Some("parent_pack"),
            &["@child_opt"],
            &[],
            &["@child_opt"],
        );

        mock.mock_pack("parent_pack", None, &["@parent_opt"], &[], &["@parent_opt"]);

        mock.expect_get_repo_path()
            .return_const(PathBuf::from("/repo"));

        let mut paths = mock.get_optional_paths("child_pack").unwrap();

        // Sort to ensure consistent order for assertions
        paths.sort();

        assert_eq!(paths.len(), 2);

        let mut expected_paths = vec![
            PathBuf::from("/repo")
                .join(get_pack_addon_directory_name("child_pack"))
                .join("@child_opt"),
            PathBuf::from("/repo")
                .join(get_pack_addon_directory_name("parent_pack"))
                .join("@parent_opt"),
        ];
        expected_paths.sort();

        assert_eq!(paths, expected_paths);
    }
}
