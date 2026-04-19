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

pub(in crate::handle) trait GetOptionalsPaths {
    fn get_optional_paths(&self, pack_name: &str) -> anyhow::Result<Vec<PathBuf>>;
}
